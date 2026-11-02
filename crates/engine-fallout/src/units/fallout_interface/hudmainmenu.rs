//! `fallout/interface/menus/hudmainmenu.cpp` (Xbox PDB source unit), subsystem `fallout/interface`: its functions in
//! FalloutNV.exe 1.4.0.525, translated (docs/ENGINE_CRATE.md).

#[allow(unused_imports)]
use crate::prelude::*;

layout! {
    /// `HUDMainMenu` (Xbox PDB), 0x278 bytes. The offsets used here are
    /// confirmed by the PC instructions in this unit.
    pub struct HUDMainMenu: 0x278 {
        0x028 pTiles: u32,
        0x184 pActive: Ptr,
        0x188 fSeenLevel: f32,
        0x194 uiMessageStartTime: u32,
        0x1a8 pCurrSubtitle: Ptr,
        0x1b8 pLastObject: Ptr,
        0x1bc iHUDVisibilityMenuModeFlags: i32,
        0x1c0 iHUDVisibilityUserSettings: i32,
        0x1c4 eMenuMode: i32,
        0x1fc bScopeVisible: bool,
        0x204 bXPMeterFinishedMoving: bool,
        0x205 bForceObjectiveReminders: bool,
        0x208 uiKillRegionDisplay: u32,
        0x20c pQuestList: Ptr,
        0x210 iCompassImageWidth: i32,
        0x214 iCompassDisplayWidth: i32,
        0x218 iCompassHalfAngle: i32,
        0x21c pCurrentEnemy: Ptr,
        0x220 bWasSneaking: bool,
        0x244 iEnemyHealthTimer: u32,
        0x248 uiQuestReminderTimer: u32,
        0x24c uiDDTdisplayTimer: i32,
        0x250 uiEnemyDDTdisplayTimer: i32,
        0x254 bKillQuestUpdates: bool,
        0x260 bQuestNameShowing: bool,
        0x261 bObjectiveNameShowing: bool,
        0x274 fShake: f32,
    }
}

fn tile(this: Ptr<HUDMainMenu>, id: u32) -> u32 {
    this.addr() + 0x28 + id * 4
}

fn current_hud(e: &Engine) -> u32 {
    e.mem.u32(0x011d_96c0)
}

fn current_player(e: &Engine) -> u32 {
    e.mem.u32(0x011d_ea3c)
}

fn hud_field(e: &Engine, offset: u32) -> u32 {
    e.mem.u32(current_hud(e) + offset)
}

fn set_hud_field(e: &mut Engine, offset: u32, value: u32) {
    let menu = current_hud(e);
    e.mem.set_u32(menu + offset, value);
}

fn create_lookup(e: &mut Engine, offset: u32, name: u32) -> u32 {
    let parent = e.call(0x00a0_9030, &args![0x3ec_u32]).u32();
    let found = e.call(0x00a0_8b20, &args![parent, name]).u32();
    set_hud_field(e, offset, found);
    found
}

fn create_child(e: &mut Engine, offset: u32, parent_offset: u32, name: u32) -> u32 {
    let parent = hud_field(e, parent_offset);
    let found = e.call(0x00a0_8b20, &args![parent, name]).u32();
    set_hud_field(e, offset, found);
    found
}

fn create_render(e: &mut Engine, menu: u32, offset: u32, parent_offset: u32, name: u32) -> u32 {
    let parent = hud_field(e, parent_offset);
    let rendered = e.call(0x00a1_ddb0, &args![menu, parent, name, 0_u32]).u32();
    set_hud_field(e, offset, rendered);
    rendered
}

fn create_render_discard(e: &mut Engine, menu: u32, parent_offset: u32, name: u32) -> u32 {
    let parent = hud_field(e, parent_offset);
    e.call(0x00a1_ddb0, &args![menu, parent, name, 0_u32]).u32()
}

fn create_brightness(e: &mut Engine, offset: u32, level: u32) {
    let current_tile = hud_field(e, offset);
    hud_main_menu_set_brightness(e, Ptr::new(current_tile), level);
}

fn create_trait(e: &mut Engine, offset: u32, trait_id: u32, value: u32) {
    let current_tile = hud_field(e, offset);
    hud_trait(e, current_tile, trait_id, value);
}

fn create_float(e: &mut Engine, offset: u32, trait_id: u32, value: f32) {
    let current_tile = hud_field(e, offset);
    hud_set_float(e, current_tile, trait_id, value);
}

fn create_get_float(e: &mut Engine, offset: u32, trait_id: u32) -> f64 {
    let current_tile = hud_field(e, offset);
    e.call(0x00a0_11b0, &args![current_tile, trait_id]).f64()
}

#[derive(Clone, Copy)]
struct HudDimensions {
    screen_width: i32,
    screen_height: i32,
    bracket_width: i32,
    bracket_height: i32,
    original_x: u32,
    total_width: u32,
    image_width: u32,
}

fn create_action_points(e: &mut Engine, menu: u32, d: HudDimensions) {
    create_lookup(e, 0x114, 0x0106_7858);
    let x = (f64::from(d.screen_width.wrapping_sub(d.bracket_width.wrapping_mul(2)))
        - create_get_float(e, 0x114, 0xfb1)
        + e.global::<f64>(0x0101_db88)) as f32;
    create_float(e, 0x114, 0xfa1, x);
    let y = (f64::from(
        d.screen_height
            .wrapping_sub(d.bracket_height.wrapping_mul(2)),
    ) - create_get_float(e, 0x114, 0xfb0)) as f32;
    create_float(e, 0x114, 0xfa2, y);
    create_trait(e, 0x114, 0xfa8, 1);
    create_render(e, menu, 0xd4, 0x114, 0x0107_3400);
    create_brightness(e, 0xd4, 2);
    create_trait(e, 0xd4, 0xfa1, (-0x25_i32) as u32);
    create_render(e, menu, 0x44, 0x114, 0x0107_33f0);
    create_trait(e, 0x44, d.original_x, 0x146);
    let meter_height: f32 = e.global(0x0107_33ec);
    create_float(e, 0x44, 0xfa2, meter_height);
    create_brightness(e, 0x44, 3);
    create_render(e, menu, 0x48, 0x114, 0x0107_33d0);
    create_trait(e, 0x48, 0xfa1, 0x13b);
    create_trait(e, 0x48, 0xfa2, 8);
    create_brightness(e, 0x48, 0);
    create_render(e, menu, 0x58, 0x114, 0x0107_33d0);
    create_trait(e, 0x58, 0xfa1, 0x13b);
    create_trait(e, 0x58, 0xfa2, 0x50);
    create_trait(e, 0x58, 0xfb9, 7);
    create_brightness(e, 0x58, 0);
    create_render(e, menu, 0x4c, 0x114, 0x0107_33d0);
    create_trait(e, 0x4c, 0xfa1, 0x50);
    let y = create_get_float(e, 0x58, 0xfa2) as f32;
    create_float(e, 0x4c, 0xfa2, y);
    create_trait(e, 0x4c, 0xfb9, 7);
    let label = e.call(0x0040_3df0, &args![0x011d_4da4_u32]).u32();
    let current_tile = hud_field(e, 0x4c);
    hud_set_text(e, current_tile, label);
    create_brightness(e, 0x4c, 0);
    create_render(e, menu, 0x50, 0x114, 0x0107_33f0);
    create_trait(e, 0x50, d.original_x, 0x5a);
    let y = (create_get_float(e, 0x58, 0xfa2) + e.global::<f64>(0x0102_1928)) as f32;
    create_float(e, 0x50, 0xfa2, y);
    let current_tile = hud_field(e, 0x50);
    e.call(
        0x00a0_1350,
        &args![current_tile, 0xfcc_u32, 0x0107_33b4_u32, 1_u32],
    );
    create_trait(e, 0x50, d.total_width, 0x3c);
    create_trait(e, 0x50, d.image_width, 1);
    create_trait(e, 0x50, 0xfb8, 200);
    create_brightness(e, 0x50, 3);
    create_render(e, menu, 0x54, 0x114, 0x0107_3398);
    let y = (create_get_float(e, 0x58, 0xfa2) + e.global::<f64>(0x0102_1928)) as f32;
    create_float(e, 0x54, 0xfa2, y);
    create_brightness(e, 0x54, 4);
    create_lookup(e, 0x180, 0x0106_ed0c);
    let first = create_get_float(e, 0x114, 0xfa1);
    let second = create_get_float(e, 0x4c, 0xfa1);
    let third = create_get_float(e, 0x50, 0xfa1);
    let fourth = create_get_float(e, 0x54, 0xfb1);
    let x = (first
        + second
        + third
        + fourth * e.global::<f64>(0x0101_de30)
        + e.global::<f64>(0x0102_40b0)) as f32;
    create_float(e, 0x180, 0xfa1, x);
    let y = (create_get_float(e, 0x114, 0xfa2) + create_get_float(e, 0x50, 0xfa2)) as f32;
    create_float(e, 0x180, 0xfa2, y);
    let height = create_get_float(e, 0x54, 0xfb0) as f32;
    create_float(e, 0x180, 0xfb0, height);
    create_trait(e, 0x180, 0xfa3, 0);
}

fn create_hit_points(e: &mut Engine, menu: u32, d: HudDimensions) {
    create_lookup(e, 0x118, 0x0107_338c);
    create_trait(
        e,
        0x118,
        0xfa1,
        d.bracket_width.wrapping_mul(2).wrapping_add(10) as u32,
    );
    let y = (f64::from(
        d.screen_height
            .wrapping_sub(d.bracket_height.wrapping_mul(2)),
    ) - create_get_float(e, 0x118, 0xfb0)) as f32;
    create_float(e, 0x118, 0xfa2, y);
    create_trait(e, 0x118, 0xfa8, 1);
    create_render(e, menu, 0xd0, 0x118, 0x0107_3374);
    create_brightness(e, 0xd0, 2);
    create_render(e, menu, 0x2c, 0x118, 0x0107_33f0);
    create_trait(e, 0x2c, d.original_x, 0x14);
    let meter_height: f32 = e.global(0x0107_33ec);
    create_float(e, 0x2c, 0xfa2, meter_height);
    create_brightness(e, 0x2c, 3);
    create_render(e, menu, 0x30, 0x118, 0x0107_33d0);
    create_trait(e, 0x30, 0xfa1, 0x35);
    create_trait(e, 0x30, 0xfa2, 8);
    create_brightness(e, 0x30, 0);
    create_render(e, menu, 0x40, 0x118, 0x0107_335c);
    create_render(e, menu, 0x34, 0x40, 0x0107_3340);
    create_trait(e, 0x34, 0xfa2, 0x1e);
    create_trait(e, 0x34, 0xfad, 4);
    create_render(e, menu, 0x38, 0x40, 0x0107_3340);
    create_trait(e, 0x38, 0xfa2, 0x1e);
    create_trait(e, 0x38, 0xfad, 3);
    create_render(e, menu, 0x3c, 0x40, 0x0107_3340);
    create_trait(e, 0x3c, 0xfa2, 10);
    create_trait(e, 0x3c, 0xfad, 2);
    let mut index = 0_i32;
    loop {
        let setting = e.call(0x0043_d4d0, &args![0x011d_32d0_u32]).u32();
        if index >= e.mem.i32(setting) {
            break;
        }
        create_render_discard(e, menu, 0x3c, 0x0107_332c);
        index = index.wrapping_add(1);
    }
    for _ in 0..10 {
        create_render_discard(e, menu, 0x38, 0x0107_3310);
    }
    create_render_discard(e, menu, 0x38, 0x0107_32f0);
    for _ in 0..10 {
        create_render_discard(e, menu, 0x34, 0x0107_32d0);
    }
}

fn create_radiation_enemy_quest(e: &mut Engine, menu: u32, d: HudDimensions) {
    create_lookup(e, 0x11c, 0x0107_32c0);
    let x = (f64::from(d.screen_width.wrapping_sub(d.bracket_width.wrapping_mul(2)))
        - create_get_float(e, 0x11c, 0xfb1)
        - e.global::<f64>(0x0102_fc70)) as f32;
    create_float(e, 0x11c, 0xfa1, x);
    create_trait(e, 0x11c, 0xfa2, d.bracket_height.wrapping_mul(2) as u32);
    create_child(e, 0x6c, 0x11c, 0x0107_32ac);
    create_child(e, 0x70, 0x11c, 0x0107_3298);
    create_child(e, 0x78, 0x11c, 0x0107_3288);
    create_child(e, 0x74, 0x11c, 0x0107_3270);
    let current_tile = hud_field(e, 0x74);
    hud_set_text(e, current_tile, 0x0107_326c);
    let text = e.call(0x0040_3df0, &args![0x011d_21d8_u32]).u32();
    let current_tile = hud_field(e, 0x78);
    hud_set_text(e, current_tile, text);

    create_lookup(e, 0x120, 0x0107_3260);
    let half_screen = d.screen_width / 2;
    let enemy_width = create_get_float(e, 0x120, 0xfb1);
    let half: f64 = e.global(0x0101_1590);
    let x = (f64::from(half_screen) - enemy_width / half) as f32;
    create_float(e, 0x120, 0xfa1, x);
    let y = (create_get_float(e, 0x118, 0xfa2) + e.global::<f64>(0x0103_5810)) as f32;
    create_float(e, 0x120, 0xfa2, y);
    create_trait(e, 0x120, 0xfa8, 1);
    create_trait(e, 0x120, 0xff4, 2);
    create_render(e, menu, 0x7c, 0x120, 0x0107_3240);
    create_brightness(e, 0x7c, 1);
    create_trait(e, 0x7c, 0xfa3, 1);
    create_render(e, menu, 0x80, 0x120, 0x0107_33f0);
    create_trait(e, 0x80, d.original_x, 0xab);
    create_trait(e, 0x80, d.total_width, 0xf0);
    create_trait(e, 0x80, 0xfa2, 3);
    create_brightness(e, 0x80, 3);
    create_trait(e, 0x80, 0xfa3, 1);
    create_render(e, menu, 0x84, 0x120, 0x0107_3220);
    create_trait(e, 0x84, 0xfa1, 0xaa);
    create_trait(e, 0x84, 0xfa2, 0x23);
    create_trait(e, 0x84, 0xfba, 0x15e);
    create_brightness(e, 0x84, 0);

    let quest_y = d.bracket_height.wrapping_mul(2).wrapping_add(0xaa);
    create_lookup(e, 0x124, 0x0107_3210);
    create_trait(
        e,
        0x124,
        0xfa1,
        d.bracket_width.wrapping_mul(2).wrapping_add(0xf) as u32,
    );
    create_trait(e, 0x124, 0xfa2, quest_y as u32);
    create_child(e, 0x5c, 0x124, 0x0107_3204);
    create_child(e, 0x60, 0x124, 0x0107_31f8);
    for _ in 0..0x40 {
        create_render_discard(e, menu, 0x60, 0x0107_31dc);
    }
    for _ in 0..0x20 {
        create_render_discard(e, menu, 0x5c, 0x0107_31c0);
    }
    for _ in 0..0x20 {
        create_render_discard(e, menu, 0x5c, 0x0107_31dc);
    }
}

fn create_text_height(e: &mut Engine, text_offset: u32) -> f32 {
    let font_size = create_get_float(e, text_offset, 0xfba) as f32;
    let font_raw = create_get_float(e, text_offset, 0xfb9);
    let font = x87_to_int(e, font_raw);
    e.with_stack(12, |e, dimensions| {
        let manager = e.call(0x005b_d5b0, &args![]).u32();
        let measured = e
            .call(
                0x00a1_b020,
                &args![manager, dimensions, 0x0107_3198_u32, font, font_size, 0_u32],
            )
            .u32();
        e.mem.f32(measured + 4)
    })
}

fn create_region_reticle_sneak_messages(e: &mut Engine, menu: u32, d: HudDimensions) {
    create_lookup(e, 0x128, 0x0107_31b0);
    create_trait(
        e,
        0x128,
        0xfa1,
        d.bracket_width.wrapping_mul(2).wrapping_add(0xf) as u32,
    );
    let y = (create_get_float(e, 0x118, 0xfa2) - e.global::<f64>(0x0107_31a8)) as f32;
    create_float(e, 0x128, 0xfa2, y);
    create_render(e, menu, 0x64, 0x128, 0x0107_31dc);
    create_brightness(e, 0x64, 0);
    create_trait(e, 0x64, 0xfa3, 1);
    let text_height = create_text_height(e, 0x64);
    create_float(e, 0x64, 0xfa2, quiet_f32(text_height));
    create_render(e, menu, 0x68, 0x128, 0x0107_31dc);
    create_brightness(e, 0x68, 0);
    create_trait(e, 0x68, 0xfa3, 1);
    create_float(e, 0x68, 0xfa2, quiet_f32(text_height));

    create_lookup(e, 0x12c, 0x0107_3188);
    let center_width = create_get_float(e, 0x12c, 0xfb1);
    let half: f64 = e.global(0x0101_1590);
    let x =
        (f64::from(d.screen_width / 2) - center_width / half - e.global::<f64>(0x0102_1928)) as f32;
    create_float(e, 0x12c, 0xfa1, x);
    let center_height = create_get_float(e, 0x12c, 0xfb0);
    let half: f64 = e.global(0x0101_1590);
    let y = (f64::from(d.screen_height / 2) - center_height / half - e.global::<f64>(0x0102_1928))
        as f32;
    create_float(e, 0x12c, 0xfa2, y);
    create_trait(e, 0x12c, 0xfa8, 1);
    create_trait(e, 0x12c, 0xfa3, 1);
    create_render(e, menu, 0xf4, 0x12c, 0x0107_3170);
    create_trait(e, 0xf4, 0xff4, 1);
    create_trait(e, 0xf4, 0xfa3, 1);

    create_lookup(e, 0x130, 0x0107_3164);
    create_trait(e, 0x130, 0xfa1, (d.screen_width / 2) as u32);
    create_trait(e, 0x130, 0xfa2, d.bracket_height.wrapping_mul(2) as u32);
    create_trait(e, 0x130, 0xfa8, 1);
    create_trait(e, 0x130, 0xfa3, 0);
    create_child(e, 0x88, 0x130, 0x0107_3158);
    create_trait(e, 0x88, 0xfa9, 0);

    create_lookup(e, 0x134, 0x0107_314c);
    create_trait(
        e,
        0x134,
        0xfa1,
        d.bracket_width.wrapping_mul(2).wrapping_sub(5) as u32,
    );
    create_trait(
        e,
        0x134,
        0xfa2,
        d.bracket_height.wrapping_mul(2).wrapping_add(0x19) as u32,
    );
    create_trait(e, 0x134, 0xfa8, 1);
    create_render(e, menu, 0x8c, 0x134, 0x0107_3134);
    create_brightness(e, 0x8c, 6);
    create_render(e, menu, 0x90, 0x134, 0x0107_31dc);
    create_brightness(e, 0x90, 0);
    for (trait_id, value) in [
        (0xfa1, 0x82),
        (0xfa2, 5),
        (0xfba, 0x140),
        (0xfad, 2),
        (0xfa9, 0),
        (0xfa3, 1),
    ] {
        create_trait(e, 0x90, trait_id, value);
    }
    create_render(e, menu, 0x94, 0x134, 0x0107_3118);
    create_brightness(e, 0x94, 1);
}

fn create_centered_text(e: &mut Engine, menu: u32, offset: u32, y: u32, label: Option<u32>) {
    create_render(e, menu, offset, 0x138, 0x0107_3220);
    create_brightness(e, offset, 0);
    let width = create_get_float(e, 0x138, 0xfb1);
    let half: f64 = e.global(0x0101_1590);
    create_float(e, offset, 0xfa1, (width / half) as f32);
    let width = create_get_float(e, 0x138, 0xfb1) as f32;
    create_float(e, offset, 0xfba, width);
    create_trait(e, offset, 0xfa2, y);
    if let Some(label_setting) = label {
        create_trait(e, offset, 0xfa9, 0);
        let label = e.call(0x0040_3df0, &args![label_setting]).u32();
        let current_tile = hud_field(e, offset);
        hud_set_text(e, current_tile, label);
        create_trait(e, offset, 0xfa3, 0);
    }
}

fn create_info_subtitles(e: &mut Engine, menu: u32, d: HudDimensions) {
    create_lookup(e, 0x138, 0x0107_3110);
    let center_width = create_get_float(e, 0x138, 0xfb1);
    let half: f64 = e.global(0x0101_1590);
    let x = (f64::from(d.screen_width / 2) - center_width / half) as f32;
    create_float(e, 0x138, 0xfa1, x);
    let y = (f64::from(
        d.screen_height
            .wrapping_sub(d.bracket_height.wrapping_mul(2)),
    ) - create_get_float(e, 0x138, 0xfb0)
        - e.global::<f64>(0x0101_db88)) as f32;
    create_float(e, 0x138, 0xfa2, y);
    create_trait(e, 0x138, 0xfa8, 1);
    create_child(e, 0x9c, 0x138, 0x0107_30f8);
    create_child(e, 0xa0, 0x9c, 0x0106_f46c);
    create_child(e, 0xa4, 0x9c, 0x0106_f47c);
    create_brightness(e, 0x9c, 0);
    let width = create_get_float(e, 0x138, 0xfb1);
    let half: f64 = e.global(0x0101_1590);
    let half_width = (width / half) as f32;
    let trait_id = e.call(0x00a0_1860, &args![0x0106_ec90_u32]).u32();
    create_float(e, 0x9c, trait_id, half_width);

    create_centered_text(e, menu, 0xa8, 0, None);
    let height = create_text_height(e, 0xa8);
    create_float(e, 0xa8, 0x100c, quiet_f32(height));
    create_centered_text(e, menu, 0xac, 0x41, Some(0x011d_2454));
    create_centered_text(e, menu, 0xb0, 0x41, Some(0x011d_1f68));
    create_render(e, menu, 0xb4, 0x138, 0x0107_33d0);
    create_brightness(e, 0xb4, 0);
    let width = create_get_float(e, 0x138, 0xfb1);
    let half: f64 = e.global(0x0101_1590);
    create_float(e, 0xb4, 0xfa1, (width / half - 25.0) as f32);
    create_trait(e, 0xb4, 0xfa2, 0x3c);
    let alpha = quiet_f32(e.mem.f32(0x011d_979c));
    create_float(e, 0xb4, 0xfa9, alpha);
    create_render(e, menu, 0xb8, 0x138, 0x0107_31dc);
    create_brightness(e, 0xb8, 0);
    create_trait(e, 0xb8, 0xfa1, 10);
    create_trait(e, 0xb8, 0xfa2, 0x3c);
    create_render(e, menu, 0xbc, 0x138, 0x0107_33d0);
    create_brightness(e, 0xbc, 0);
    let width = create_get_float(e, 0x138, 0xfb1);
    create_float(e, 0xbc, 0xfa1, (width - 30.0) as f32);
    create_trait(e, 0xbc, 0xfa2, 0x3c);
    let alpha = quiet_f32(e.mem.f32(0x011d_979c));
    create_float(e, 0xbc, 0xfa9, alpha);
    create_render(e, menu, 0xc0, 0x138, 0x0107_31dc);
    create_brightness(e, 0xc0, 0);
    let width = create_get_float(e, 0x138, 0xfb1);
    let half: f64 = e.global(0x0101_1590);
    create_float(e, 0xc0, 0xfa1, (width / half + 5.0) as f32);
    create_trait(e, 0xc0, 0xfa2, 0x3c);
    create_render(e, menu, 0xc4, 0x138, 0x0107_30e0);
    create_brightness(e, 0xc4, 2);

    create_lookup(e, 0x13c, 0x0107_30d4);
    let width = create_get_float(e, 0x13c, 0xfb1);
    let half: f64 = e.global(0x0101_1590);
    create_float(
        e,
        0x13c,
        0xfa1,
        (f64::from(d.screen_width / 2) - width / half) as f32,
    );
    // 0076dfad: the y read is the info box's (hud+0x138); the height and the
    // write are the subtitle tile's (hud+0x13c).
    let top = create_get_float(e, 0x138, 0xfa2);
    let height = create_get_float(e, 0x13c, 0xfb0);
    create_float(e, 0x13c, 0xfa2, (top - height - 50.0) as f32);
    create_trait(e, 0x13c, 0xfa8, 1);
    create_render(e, menu, 0x98, 0x13c, 0x0107_3220);
    create_brightness(e, 0x98, 0);
    let width = create_get_float(e, 0x13c, 0xfb1);
    let half: f64 = e.global(0x0101_1590);
    create_float(e, 0x98, 0xfa1, (width / half) as f32);
    let width = create_get_float(e, 0x13c, 0xfb1) as f32;
    create_float(e, 0x98, 0xfba, width);
}

fn create_hotkeys_xp_breath_explosives(e: &mut Engine, menu: u32, d: HudDimensions) {
    create_lookup(e, 0x140, 0x0107_30cc);
    let x = (create_get_float(e, 0x114, 0xfa1) + e.global::<f64>(0x0101_5a38)) as f32;
    create_float(e, 0x140, 0xfa1, x);
    let y = (f64::from(
        d.screen_height
            .wrapping_sub(d.bracket_height.wrapping_mul(2)),
    ) - create_get_float(e, 0x118, 0xfb0)
        - create_get_float(e, 0x140, 0xfb0)
        - e.global::<f64>(0x0102_fc70)) as f32;
    create_float(e, 0x140, 0xfa2, y);
    create_trait(e, 0x140, 0xfa8, 1);
    create_render(e, menu, 0xc8, 0x140, 0x0107_30b0);
    let current_menu = current_hud(e);
    let selector = hud_field(e, 0xc8);
    e.call(0x0070_1610, &args![current_menu + 0x1c8, selector]);
    create_render(e, menu, 0xcc, 0x140, 0x0107_3220);
    let width = create_get_float(e, 0x140, 0xfb1);
    let half: f64 = e.global(0x0101_1590);
    create_float(e, 0xcc, 0xfa1, (width / half) as f32);
    create_trait(e, 0xcc, 0xfa2, (-0x32_i32) as u32);
    create_trait(e, 0xcc, 0xfa3, 0);

    create_lookup(e, 0x144, 0x0107_30a8);
    let x = d
        .screen_width
        .wrapping_sub(d.bracket_width.wrapping_mul(2))
        .wrapping_sub(0x186);
    let y = d
        .screen_height
        .wrapping_sub(d.bracket_height.wrapping_mul(2))
        .wrapping_sub(0x118);
    create_trait(e, 0x144, 0xfa1, x as u32);
    create_trait(e, 0x144, 0xfa2, y as u32);
    create_trait(e, 0x144, 0x1007, y as u32);
    for (offset, name) in [
        (0xdc, 0x0107_309c),
        (0xe0, 0x0107_3094),
        (0xe4, 0x0107_3088),
        (0xd8, 0x0107_307c),
        (0xe8, 0x0107_3070),
        (0xec, 0x0107_3064),
        (0xf0, 0x0107_3058),
    ] {
        create_child(e, offset, 0x144, name);
    }

    create_lookup(e, 0x148, 0x0107_304c);
    create_trait(e, 0x148, 0xfa1, (d.screen_width / 2) as u32);
    let y = (create_get_float(e, 0x130, 0xfa2) + e.global::<f64>(0x0107_2310)) as f32;
    create_float(e, 0x148, 0xfa2, y);
    create_lookup(e, 0x14c, 0x0107_3030);
    create_trait(e, 0x14c, 0xfa1, (d.screen_width / 2) as u32);
    create_trait(e, 0x14c, 0xfa2, (d.screen_height / 2) as u32);
    for _ in 0..6 {
        create_render_discard(e, menu, 0x14c, 0x0107_3010);
    }
}

fn cripple_scale(e: &Engine, size: f32, numerator: u32) -> f32 {
    (f64::from(size) * e.global::<f64>(numerator) / e.global::<f64>(0x0101_7a40)) as f32
}

fn create_crippled_part(
    e: &mut Engine,
    offset: u32,
    name: u32,
    size: f32,
    width: u32,
    height: u32,
) {
    create_child(e, offset, 0x150, name);
    create_float(e, offset, 0xfb8, quiet_f32(size));
    let scaled_width = cripple_scale(e, size, width);
    create_float(e, offset, 0xfb1, scaled_width);
    let scaled_height = cripple_scale(e, size, height);
    create_float(e, offset, 0xfb0, scaled_height);
}

fn create_crippled_limbs(e: &mut Engine, d: HudDimensions) -> f32 {
    create_lookup(e, 0x150, 0x0107_2ff8);
    let size: f32 = quiet_f32(e.global(0x0102_f0f8));
    create_trait(
        e,
        0x150,
        0xfa1,
        d.screen_width
            .wrapping_sub(d.bracket_width.wrapping_mul(2))
            .wrapping_sub(200) as u32,
    );
    create_trait(
        e,
        0x150,
        0xfa2,
        d.bracket_height.wrapping_mul(2).wrapping_add(100) as u32,
    );

    create_crippled_part(e, 0xfc, 0x0101_f604, size, 0x0107_2ff0, 0x0107_2fe8);
    create_trait(e, 0xfc, 0xfa1, 0);
    create_trait(e, 0xfc, 0xfa2, 0x28);
    create_crippled_part(e, 0xf8, 0x0105_c2c8, size, 0x0107_2fe0, 0x0102_40c0);
    let head_x = create_get_float(e, 0xfc, 0xfa1);
    let head_width = create_get_float(e, 0xfc, 0xfb1);
    let torso_width = create_get_float(e, 0xf8, 0xfb1);
    let half: f64 = e.global(0x0101_1590);
    create_float(
        e,
        0xf8,
        0xfa1,
        (head_x + (head_width - torso_width) / half) as f32,
    );
    let head_y = create_get_float(e, 0xfc, 0xfa2);
    let head_height = create_get_float(e, 0xfc, 0xfb0);
    let torso_height = create_get_float(e, 0xf8, 0xfb0);
    let half: f64 = e.global(0x0101_1590);
    let offset = f64::from(cripple_scale(e, size, 0x0102_40b8));
    create_float(
        e,
        0xf8,
        0xfa2,
        (head_y + (head_height - torso_height) / half + offset) as f32,
    );

    create_crippled_part(e, 0x100, 0x0107_2fd8, size, 0x0107_04e8, 0x0102_e430);
    let x = create_get_float(e, 0xfc, 0xfa1);
    let shift = f64::from(cripple_scale(e, size, 0x0107_2fd0));
    create_float(e, 0x100, 0xfa1, (x + shift) as f32);
    let y = create_get_float(e, 0xfc, 0xfa2);
    let shift = f64::from(cripple_scale(e, size, 0x0107_2fc8));
    create_float(e, 0x100, 0xfa2, (y + shift) as f32);

    create_crippled_part(e, 0x104, 0x0107_2fb8, size, 0x0101_7a40, 0x0107_2fb0);
    let x = create_get_float(e, 0x100, 0xfa1);
    let shift = f64::from(cripple_scale(e, size, 0x0106_6808));
    create_float(e, 0x104, 0xfa1, (x + shift) as f32);
    let y = create_get_float(e, 0x100, 0xfa2) as f32;
    create_float(e, 0x104, 0xfa2, y);

    create_crippled_part(e, 0x108, 0x0107_2fa0, size, 0x0107_2f98, 0x0107_2f90);
    let x = create_get_float(e, 0x100, 0xfa1);
    let shift = f64::from(cripple_scale(e, size, 0x0107_2f88));
    create_float(e, 0x108, 0xfa1, (x + shift) as f32);
    let y = create_get_float(e, 0x100, 0xfa2);
    let shift = f64::from(cripple_scale(e, size, 0x0107_2f80));
    create_float(e, 0x108, 0xfa2, (y + shift) as f32);

    create_crippled_part(e, 0x10c, 0x0107_2f70, size, 0x0107_2f68, 0x0106_ebd0);
    let x = create_get_float(e, 0x100, 0xfa1);
    let shift = f64::from(cripple_scale(e, size, 0x0103_57a8));
    create_float(e, 0x10c, 0xfa1, (x + shift) as f32);
    let y = create_get_float(e, 0x100, 0xfa2);
    let shift = f64::from(cripple_scale(e, size, 0x0107_2fc8));
    create_float(e, 0x10c, 0xfa2, (y + shift) as f32);

    create_crippled_part(e, 0x110, 0x0107_2f58, size, 0x0107_2f50, 0x0106_ebd0);
    let x = create_get_float(e, 0x100, 0xfa1);
    let shift = f64::from(cripple_scale(e, size, 0x0107_2f48));
    create_float(e, 0x110, 0xfa1, (x + shift) as f32);
    let y = create_get_float(e, 0x100, 0xfa2);
    let shift = f64::from(cripple_scale(e, size, 0x0107_2f40));
    create_float(e, 0x110, 0xfa2, (y + shift) as f32);
    size
}

fn create_ddt_and_hardcore(e: &mut Engine, icon_size: f32) {
    for (offset, name, visible) in [
        (0x154, 0x0107_2f38, 0_u32),
        (0x158, 0x0107_2f28, 1),
        (0x170, 0x0107_2f38, 1),
        (0x174, 0x0107_2f18, 1),
    ] {
        create_lookup(e, offset, name);
        if matches!(offset, 0x154 | 0x170) {
            let first_x = create_get_float(e, 0x118, 0xfa1);
            let second_x = create_get_float(e, 0x118, 0xfb1);
            let shift: f64 = e.global(0x0102_0998);
            create_float(e, offset, 0xfa1, (first_x + second_x + shift) as f32);
            let first_y = create_get_float(e, 0x114, 0xfa2);
            let second_y = create_get_float(e, 0x40, 0xfa2);
            let shift: f64 = e.global(0x0102_40b8);
            create_float(e, offset, 0xfa2, (first_y + second_y - shift) as f32);
        } else {
            let first_x = create_get_float(e, 0x120, 0xfa1);
            let second_x = create_get_float(e, 0x80, 0xfa1);
            let shift: f64 = e.global(0x0102_f070);
            create_float(e, offset, 0xfa1, (first_x + second_x - shift) as f32);
            let first_y = create_get_float(e, 0x120, 0xfa2);
            let second_y = create_get_float(e, 0x80, 0xfa2);
            create_float(e, offset, 0xfa2, (first_y + second_y) as f32);
        }
        create_brightness(e, offset, 0);
        create_trait(e, offset, 0xfa3, visible);
        if matches!(offset, 0x158 | 0x174) {
            create_trait(e, offset, 0xff4, 2);
        }
    }
    create_lookup(e, 0x15c, 0x0107_2f08);
    let first_x = create_get_float(e, 0x114, 0xfa1);
    let second_x = create_get_float(e, 0x58, 0xfa1);
    let shift: f64 = e.global(0x0107_2f00);
    create_float(e, 0x15c, 0xfa1, (first_x + second_x - shift) as f32);
    let first_y = create_get_float(e, 0x114, 0xfa2);
    let second_y = create_get_float(e, 0x58, 0xfa2);
    create_float(e, 0x15c, 0xfa2, (first_y + second_y) as f32);
    create_brightness(e, 0x15c, 0);
    create_trait(e, 0x15c, 0xfa3, 1);

    create_lookup(e, 0x160, 0x0107_2eec);
    let x = (create_get_float(e, 0x118, 0xfa1) + e.global::<f64>(0x0106_c948)) as f32;
    create_float(e, 0x160, 0xfa1, x);
    let y = create_get_float(e, 0x118, 0xfa2) as f32;
    create_float(e, 0x160, 0xfa2, y);
    create_brightness(e, 0x160, 0);
    create_trait(e, 0x160, 0xfa3, 1);
    create_trait(e, 0x160, 0xfa8, 1);
    create_child(e, 0x164, 0x160, 0x0106_75c0);
    create_trait(e, 0x164, 0xfa1, 0);
    create_trait(e, 0x164, 0xfa2, 0);
    create_float(e, 0x164, 0xfb8, quiet_f32(icon_size));
    create_trait(e, 0x164, 0xfa9, 0);
    for (offset, name, predecessors) in [
        (0x16c, 0x0106_75b8, &[0x164_u32][..]),
        (0x168, 0x0106_37d4, &[0x16c, 0x164][..]),
        (0x178, 0x0105_afd0, &[0x16c, 0x164, 0x168][..]),
        (0x17c, 0x0107_2ee4, &[0x16c, 0x164, 0x168, 0x178][..]),
    ] {
        create_child(e, offset, 0x160, name);
        let mut x = 0.0;
        for &previous in predecessors {
            x += create_get_float(e, previous, 0xfb1);
            x += e.global::<f64>(0x0102_0758);
        }
        create_float(e, offset, 0xfa1, x as f32);
        create_trait(e, offset, 0xfa2, 0);
        create_float(e, offset, 0xfb8, quiet_f32(icon_size));
        create_trait(e, offset, 0xfa9, 0);
    }
}

fn create_scene(e: &mut Engine, menu: u32) {
    // The PC body at 0076fbd3-0076ff9a builds the compass texture,
    // camera, and optional forward animation after the XML tiles exist.
    e.with_stack(0x380, |e, storage| {
        let cancellation = storage.addr();
        let texture_name = storage.addr() + 0x80;
        let matrix_x = storage.addr() + 0x100;
        let matrix_y = storage.addr() + 0x160;
        let matrix_product = storage.addr() + 0x1c0;
        let vector = storage.addr() + 0x200;
        let second_vector = storage.addr() + 0x220;
        let controller_name = storage.addr() + 0x240;

        e.call(0x00a0_4200, &args![0_u32]);
        e.call(0x0063_3c90, &args![cancellation, 0_u32]);
        let name = e
            .call(0x0043_8170, &args![texture_name, 0x0107_2e40_u32])
            .u32();
        let texture = e
            .call(0x00a5_fd70, &args![name, 0x011a_9598_u32, 1_u32, 0_u32])
            .u32();
        e.call(0x0066_b0d0, &args![cancellation, texture]);
        e.call(0x0043_81b0, &args![texture_name]);

        let image = e.call(0x0055_9450, &args![cancellation]).u32();
        let tile = hud_field(e, 0x40);
        let object = e.vcall(tile, 0x20, &args![]).u32();
        fn_007700b0(e, Ptr::new(object), Ptr::new(image));
        let tile = hud_field(e, 0x40);
        let object = e.vcall(tile, 0x20, &args![]).u32();
        fn_007700d0(e, Ptr::new(object), 1);

        let allocation = e.call(0x00aa_13e0, &args![0xac_u32]).u32();
        let attachment = if allocation == 0 {
            0
        } else {
            e.call(0x00a5_ecb0, &args![allocation, 0_u32]).u32()
        };
        e.call(0x0066_b0d0, &args![0x011d_9770_u32, attachment]);
        e.call(0x0068_15c0, &args![matrix_x]);
        e.call(0x0068_15c0, &args![matrix_y]);
        let rotation = quiet_f32(e.global(0x0101_ff38));
        e.call(0x0052_4ac0, &args![matrix_x, rotation]);
        let rotation = quiet_f32(e.global(0x0101_ff38));
        e.call(0x004a_0c90, &args![matrix_y, rotation]);
        let product = e
            .call(0x0043_f8d0, &args![matrix_x, matrix_product, matrix_y])
            .u32();
        let scene = e.call(0x0055_9450, &args![0x011d_9770_u32]).u32();
        e.call(0x0043_fa80, &args![scene, product]);
        let distance = quiet_f32(e.global(0x0101_e2bc));
        let scene = e.call(0x0055_9450, &args![0x011d_9770_u32]).u32();
        e.call(0x0044_0490, &args![scene, distance]);
        let field_of_view = quiet_f32(e.global(0x0107_2e3c));
        let scene = e.call(0x0055_9450, &args![0x011d_9770_u32]).u32();
        e.call(0x004b_c1f0, &args![scene, field_of_view, 0.0_f32, 0.0_f32]);
        e.call(0x0043_d410, &args![vector, 0.0_f32, 0_u32, 0_u32]);
        let scene = e.call(0x0055_9450, &args![0x011d_9770_u32]).u32();
        e.call(0x00a5_9c60, &args![scene, vector]);

        let current_menu = current_hud(e);
        let camera = e.call(0x0077_ee50, &args![current_menu]).u32();
        e.call(0x0066_b0d0, &args![0x011d_975c_u32, camera]);
        let heap_object = e.call(0x0040_1000, &args![1_u32]).u32();
        e.mem.set_u32(0x011d_96bc, heap_object);
        let audio = e.call(0x0045_0b80, &args![1_u32]).u32();
        let scene = e.call(0x0055_9450, &args![0x011d_9770_u32]).u32();
        e.vcall(audio, 0xdc, &args![scene, 1_u32]);
        let scene = e.call(0x0055_9450, &args![0x011d_9770_u32]).u32();
        let bound = e.call(0x0043_d450, &args![scene]).u32();
        let distance = e.call(0x0084_d030, &args![bound]).f64();
        let shadow_distance = (distance * e.global::<f64>(0x0102_1928)) as f32;
        let scene = e.call(0x0055_9450, &args![0x011d_9770_u32]).u32();
        let audio = e.call(0x0045_0b80, &args![1_u32]).u32();
        e.call(0x00b5_ca70, &args![audio, scene, shadow_distance]);

        let tile = hud_field(e, 0x148);
        let node = e.call(0x0056_c7f0, &args![tile]).u32();
        let object = e.call(0x0043_b4a0, &args![node, 0_u32]).u32();
        let controller = e.call(0x00a5_c570, &args![object, 0x011f_36ac_u32]).u32();
        let left = quiet_f32(e.global(0x0101_6b78));
        e.call(0x0064_04b0, &args![node, left, 1.0_f32, 0.0_f32, 0.0_f32]);
        e.call(0x0043_d410, &args![second_vector, 0.0_f32, 1_u32, 0_u32]);
        let object = e.call(0x0043_b4a0, &args![node, 0_u32]).u32();
        e.call(0x00a5_9c60, &args![object, second_vector]);
        if controller != 0 {
            let name = e
                .call(0x0043_8170, &args![controller_name, 0x0104_8458_u32])
                .u32();
            let forward = e.call(0x0047_a520, &args![controller, name]).u32();
            set_hud_field(e, 0x200, forward);
            e.call(0x0043_81b0, &args![controller_name]);
            e.call(0x0048_fef0, &args![controller, 0.0_f32]);
            if hud_field(e, 0x200) != 0 {
                let forward = hud_field(e, 0x200);
                e.call(
                    0x0047_aab0,
                    &args![controller, forward, 0_u32, 1_u32, 1.0_f32, 0.0_f32, 0_u32],
                );
            }
        }
        e.call(0x00a1_dc20, &args![menu, 1_u32]);
        let setting = e.call(0x0040_3e20, &args![0x011d_9774_u32]).u32();
        let alpha = quiet_f32(e.mem.f32(setting));
        e.call(0x0077_e430, &args![alpha]);
        hud_main_menu_set_menu_mode(e, 5);
        let critical = e.call(0x0071_3d70, &args![]).u32();
        e.call(0x0045_38c0, &args![critical]);
        e.call(0x0045_cec0, &args![cancellation]);
    });
}

// Translated from 0076bfe0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::Create` (Xbox PDB): loads the XML menu, resolves its tiles,
/// lays out HUD elements, and builds the compass scene.
// Translation incomplete: 0076bfe0 - the compiler's SEH unwind paths are omitted.
pub fn hud_main_menu_create(e: &mut Engine) -> Ptr {
    if e.call(0x004b_7210, &args![]).u32() == 0 {
        return Ptr::NULL;
    }
    let menu_class = e.call(0x0070_6f20, &args![]).u32();
    let previous = e.call(0x00a0_9030, &args![menu_class]).u32();
    if previous != 0 {
        e.vcall(previous, 0, &args![1_u32]);
    }
    let resource = e.call(0x004b_7210, &args![]).u32();
    let maximum_depth = e.call(0x00a0_1530, &args![]).f64() as f32;
    let critical = e.call(0x0071_3d70, &args![]).u32();
    e.call(0x0045_38a0, &args![critical, 0_u32]);
    // 0076c092: the pushed path is Tile::ReadFile's word; 00586150 (RET, no word) gives its `this`.
    let tiles = e.call(0x0058_6150, &args![resource]).u32();
    let xml = e.call(0x00a0_1b00, &args![tiles, 0x0107_343c_u32]).u32();
    let menu = e.call(0x00a0_3c90, &args![xml]).u32();
    let valid_menu = if menu == 0 {
        false
    } else {
        let class = e.vcall(menu, 0x34, &args![]).u32();
        let expected = e.call(0x0070_6f20, &args![]).u32();
        class == expected
    };
    if !valid_menu {
        if menu != 0 && e.call(0x0072_6070, &args![menu]).u32() != 0 {
            e.vcall(menu, 0, &args![1_u32]);
        }
        let critical = e.call(0x0071_3d70, &args![]).u32();
        e.call(0x0045_38c0, &args![critical]);
        return Ptr::NULL;
    }
    e.call(0x00a1_dc70, &args![menu, xml, 0_u32]);
    e.mem.set_u32(0x011d_96c0, menu);
    let screen_width = e.call(0x0071_5d40, &args![]).f64();
    let screen_width = x87_to_int(e, screen_width);
    let screen_height = e.call(0x0071_5da0, &args![]).f64();
    let screen_height = x87_to_int(e, screen_height);
    let bracket_width = e.call(0x0071_77c0, &args![]).f64();
    let bracket_width = x87_to_int(e, bracket_width);
    let bracket_height = e.call(0x0071_7820, &args![]).f64();
    let bracket_height = x87_to_int(e, bracket_height);
    let original_x = e.call(0x00a0_0940, &args![0x0107_3430_u32, u32::MAX]).u32();
    let total_width = e.call(0x00a0_0940, &args![0x0107_3424_u32, u32::MAX]).u32();
    let image_width = e.call(0x00a0_0940, &args![0x0107_3418_u32, u32::MAX]).u32();
    let dimensions = HudDimensions {
        screen_width,
        screen_height,
        bracket_width,
        bracket_height,
        original_x,
        total_width,
        image_width,
    };
    create_action_points(e, menu, dimensions);
    create_hit_points(e, menu, dimensions);
    create_radiation_enemy_quest(e, menu, dimensions);
    create_region_reticle_sneak_messages(e, menu, dimensions);
    create_info_subtitles(e, menu, dimensions);
    create_hotkeys_xp_breath_explosives(e, menu, dimensions);
    let icon_size = create_crippled_limbs(e, dimensions);
    create_ddt_and_hardcore(e, icon_size);

    let current_menu = current_hud(e);
    if !fn_00770070(e, Ptr::new(current_menu)) {
        e.call(0x0040_fbe0, &args![0x0107_2e70_u32]);
        e.call(0x0070_37e0, &args![]);
    }
    create_maximum_depth(e, xml, maximum_depth);
    for (tile_offset, setting) in [
        (0x58, 0x011d_4918),
        (0x48, 0x011d_3e8c),
        (0x30, 0x011d_300c),
    ] {
        let text = e.call(0x0040_3df0, &args![setting]).u32();
        let current_tile = hud_field(e, tile_offset);
        hud_set_text(e, current_tile, text);
    }
    set_hud_field(e, 0x210, 0x358);
    let compass = create_get_float(e, 0x40, 0xfb1);
    let display_width = x87_to_int(e, compass / e.global::<f64>(0x0101_1590)).wrapping_sub(0x23);
    set_hud_field(e, 0x214, display_width as u32);
    set_hud_field(e, 0x218, 0x46);
    hud_trait(e, xml, 0xfa3, 1);
    let current_menu = current_hud(e);
    e.call(0x0072_6070, &args![current_menu]);
    create_scene(e, menu);
    Ptr::new(xml)
}

/// PC 0076fabb–0076fb00: the root XML tile gets the maximum depth (0xfad) when
/// its 0xfa7 value equals either constant. Each getter and the setter take
/// ECX = [EBP-0x14], the tile `Tile::ReadFile` (00a01b00) returned, not the
/// HUD's 0x40 tile.
fn create_maximum_depth(e: &mut Engine, xml: u32, maximum_depth: f32) {
    let kind = e.call(0x00a0_11b0, &args![xml, 0xfa7_u32]).f64();
    let first: f64 = e.global(0x0107_04f0);
    let second_match = if kind == first {
        false
    } else {
        e.call(0x00a0_11b0, &args![xml, 0xfa7_u32]).f64() == e.global::<f64>(0x0107_04e8)
    };
    if kind == first || second_match {
        hud_set_float(e, xml, 0xfad, quiet_f32(maximum_depth));
    }
}

fn hud_tile(e: &Engine, id: u32) -> u32 {
    e.mem.u32(e.mem.u32(0x011d_96c0) + 0x28 + id * 4)
}

fn hud_trait(e: &mut Engine, hud_tile: u32, trait_id: u32, value: u32) {
    e.call(0x0070_0320, &args![hud_tile, trait_id, value]);
}

fn hud_float(e: &mut Engine, hud_tile: u32, trait_id: u32) -> f32 {
    e.call(0x00a0_11b0, &args![hud_tile, trait_id]).f32()
}

fn hud_set_float(e: &mut Engine, hud_tile: u32, trait_id: u32, value: f32) {
    e.call(0x00a0_12d0, &args![hud_tile, trait_id, value, 1_u32]);
}

fn hud_set_text(e: &mut Engine, hud_tile: u32, text: u32) {
    e.call(0x00a0_1350, &args![hud_tile, 0xfc4_u32, text, 1_u32]);
}

fn set_info_target_string(e: &mut Engine) {
    let target_tile = hud_field(e, 0xf4);
    let text = e.call(0x00a0_11f0, &args![target_tile, 0x1004_u32]).u32();
    let target_tile = hud_field(e, 0xf4);
    e.call(0x00a0_1350, &args![target_tile, 0xfcc_u32, text, 1_u32]);
}

fn set_info_target_prompt(e: &mut Engine, mode: u32, string_trait: u32) {
    let target_tile = hud_field(e, 0xf4);
    hud_trait(e, target_tile, 0xff4, mode);
    let target_tile = hud_field(e, 0xf4);
    let text = e.call(0x00a0_11f0, &args![target_tile, string_trait]).u32();
    let target_tile = hud_field(e, 0xf4);
    e.call(0x00a0_1350, &args![target_tile, 0xfcc_u32, text, 1_u32]);
}

fn set_info_unsupported_form(e: &mut Engine) -> bool {
    set_info_target_prompt(e, 1, 0x1004);
    for offset in [
        0xa8_u32, 0x9c, 0x7c, 0x80, 0x84, 0xac, 0xb0, 0xb4, 0xb8, 0xbc, 0xc0, 0xc4,
    ] {
        let target_tile = hud_field(e, offset);
        hud_trait(e, target_tile, 0xfa3, 0);
    }
    true
}

fn set_info_player_action(e: &mut Engine, action: u32) {
    let duration: f32 = quiet_f32(e.global(0x0101_7b78));
    let player = current_player(e);
    e.call(0x0096_3eb0, &args![player, action, duration, 0_u32]);
    e.call(0x0097_81a0, &args![0x011e_0e80_u32]);
}

fn set_info_locked_owner_action(e: &mut Engine, reference: Ptr, lock: u32) {
    if lock != 0 && e.call(0x0050_21a0, &args![lock]).bool() {
        let owner = e.call(0x0056_7790, &args![reference]).u32();
        if owner != 0
            && (e.mem.u32(lock + 4) == 0 || {
                let key = e.mem.u32(lock + 4);
                let player = current_player(e);
                e.call(0x0057_5610, &args![player, key]).i32() < 1
            })
        {
            set_info_player_action(e, 0xb);
        }
    }
}

fn set_info_lock_text_alpha(e: &mut Engine, setting: u32) {
    // 007769bc / 00776a2c: `00403df0` takes no stack word (plain RET); the pushed 1 is
    // the last word of `00a01350` (RET 0xc), which `hud_set_text` passes.
    let text = e.call(0x0040_3df0, &args![setting]).u32();
    let lock_tile = hud_field(e, 0xac);
    hud_set_text(e, lock_tile, text);
    set_info_lock_alpha(e);
}

fn set_info_lock_alpha(e: &mut Engine) {
    set_info_tile_alpha(e, 0xac);
}

fn set_info_tile_alpha(e: &mut Engine, offset: u32) {
    // 007769de / 00776a4e / 00776be5 / 00776d04: `00403e20` takes no stack word (plain
    // RET); the pushed 1 is the last word of `00a012d0` (RET 0xc), which `hud_set_float`
    // passes.
    let opacity_setting = e.call(0x0040_3e20, &args![0x011d_9774_u32]).u32();
    let opacity = quiet_f32(e.mem.f32(opacity_setting));
    let alpha = (f64::from(opacity) * e.global::<f64>(0x0101_e568)) as f32;
    let target_tile = hud_field(e, offset);
    hud_set_float(e, target_tile, 0xfa9, alpha);
}

fn set_info_action_setting(e: &mut Engine, setting: u32) {
    let text = e.call(0x0040_3df0, &args![setting]).u32();
    let tile = hud_field(e, 0x9c);
    hud_set_text(e, tile, text);
}

/// PC 00776fb8–0077704d: destination shown next to a teleporting door.
fn set_info_teleport_name(e: &mut Engine, teleport: u32) {
    e.with_stack(8, |e, destination| {
        e.call(0x0040_37b0, &args![destination]);
        e.call(0x0043_a2e0, &args![teleport, destination]);
        let destination_name = e.call(0x0055_9450, &args![destination]).u32();
        let separator = e.call(0x0040_3df0, &args![0x011d_3738_u32]).u32();
        e.with_stack(0x104, |e, buffer| {
            e.call(
                0x0040_6d00,
                &args![
                    buffer,
                    0x104_u32,
                    0x0107_34e0_u32,
                    0x011d_9c48_u32,
                    separator,
                    destination_name
                ],
            );
            let name_tile = hud_field(e, 0xa8);
            hud_set_text(e, name_tile, buffer.addr());
        });
        e.call(0x0040_37d0, &args![destination]);
    });
}

/// The action text and button glyph portion of `SetInfoForRef`, PC
/// 007773fe–00777c90. The string's lifetime ends before item details begin.
#[allow(clippy::too_many_arguments)] // Keep PC 007773fe live locals explicit at this branch boundary.
fn set_info_inventory_action(
    e: &mut Engine,
    reference: Ptr,
    inventory: u32,
    target_type: u32,
    form_type: u32,
    actor: bool,
    lock: u32,
    can_ignore_lock: bool,
    display_mask: &mut u32,
) {
    e.with_stack(8, |e, action| {
        e.call(0x0040_c0e0, &args![action, 0x0101_1584_u32]);
        if inventory != 0 {
            e.call(0x0050_1c10, &args![action, inventory + 0x18]);
        }
        if e.call(0x0040_48e0, &args![action]).u32() == 0 {
            let base = e.call(0x007a_f430, &args![reference]).u32();
            let cast = e
                .call(
                    0x00ec_43fb,
                    &args![base, 0_u32, 0x0118_3108_u32, 0x0118_6584_u32, 0_u32],
                )
                .u32();
            if cast != 0 {
                e.with_stack(8, |e, name| {
                    let converted = e.call(0x0051_10e0, &args![cast, name]).u32();
                    e.call(0x0050_1c10, &args![action, converted]);
                    e.call(0x0040_37d0, &args![name]);
                });
            }
        }
        if e.call(0x0040_48e0, &args![action]).u32() != 0 {
            let text = e.call(0x0055_9450, &args![action]).u32();
            let tile = hud_field(e, 0x9c);
            hud_set_text(e, tile, text);
            let name_tile = hud_field(e, 0xa8);
            hud_set_text(e, name_tile, 0x0101_1584);
            *display_mask |= 2;
        } else if target_type == 1 && e.call(0x0057_9690, &args![reference]).bool() {
            let setting = if actor && e.call(0x0056_6950, &args![reference]).bool() {
                // 0077754f: MOV ECX,[0x011d517c] loads the Setting stored in table slot
                // 011d5160[7]; the other two settings are immediates (B9 imm32).
                e.mem.u32(0x011d_517c)
            } else if form_type == 0x2a {
                0x011d_3708
            } else if form_type == 0x2b {
                let process = e.call(0x0041_81e0, &args![reference]).u32();
                if e.vcall(process + 0x30, 0x18, &args![]).bool() {
                    0x011d_3708
                } else {
                    0x011d_4f48
                }
            } else {
                0x011d_4f48
            };
            set_info_action_setting(e, setting);
            *display_mask |= 2;
        } else {
            let first_base = e.call(0x007a_f430, &args![reference]).u32();
            let special = first_base == e.mem.u32(0x011c_a27c)
                || {
                    let second_base = e.call(0x007a_f430, &args![reference]).u32();
                    second_base == e.mem.u32(0x011c_a280)
                }
                || (target_type == 2 && (form_type == 0x2a || form_type == 0x2b));
            if special {
                set_info_action_setting(e, 0x011d_38dc);
                *display_mask |= 2;
            } else if target_type == 8
                && form_type == 0x1c
                && !e.call(0x0057_b460, &args![reference]).bool()
            {
                let first_state = e.call(0x0047_b250, &args![reference]).u32();
                let open = first_state == 1 || e.call(0x0047_b250, &args![reference]).u32() == 2;
                let setting = if lock != 0
                    && e.call(0x0050_21a0, &args![lock]).bool()
                    && {
                        let player = current_player(e);
                        !e.call(0x0057_85e0, &args![reference, player, 1_u32]).bool()
                    }
                    && !can_ignore_lock
                {
                    0x011d_45a0
                } else if open {
                    0x011d_22b0
                } else {
                    e.mem
                        .u32(0x011d_5160_u32.wrapping_add(target_type.wrapping_mul(4)))
                };
                set_info_action_setting(e, setting);
                *display_mask |= 2;
            } else if target_type == 4 && form_type == 0x2b {
                *display_mask &= !3;
            } else if target_type == 6 {
                let setting = e.mem.u32(0x011d_5164);
                set_info_action_setting(e, setting);
                *display_mask |= 2;
            } else if target_type != 0
                && (target_type as i32) < 0x14
                && !e.call(0x0057_b460, &args![reference]).bool()
            {
                let essential = target_type == 1
                    && actor
                    && {
                        let player = current_player(e);
                        e.call(0x0049_97b0, &args![player]).bool()
                    }
                    && e.call(0x0087_f3d0, &args![reference]).bool();
                if !essential {
                    let setting = e
                        .mem
                        .u32(0x011d_5160_u32.wrapping_add(target_type.wrapping_mul(4)));
                    set_info_action_setting(e, setting);
                    *display_mask |= 2;
                }
            } else if e.vcall(reference.addr(), 0x224, &args![]).bool() {
                let process = e.call(0x0041_81e0, &args![reference]).u32();
                if e.call(0x0045_cd80, &args![process]).f64() != e.global::<f64>(0x0101_2060) {
                    let setting = if e.call(0x0044_4d00, &args![reference, 0x200_u32]).bool() {
                        e.mem.u32(0x011d_5164)
                    } else {
                        0x011d_3588
                    };
                    set_info_action_setting(e, setting);
                    *display_mask |= 2;
                    let target_tile = hud_field(e, 0xf4);
                    hud_trait(e, target_tile, 0xff4, 1);
                    let target_tile = hud_field(e, 0xf4);
                    let text = e.call(0x00a0_11f0, &args![target_tile, 0x1005_u32]).u32();
                    let target_tile = hud_field(e, 0xf4);
                    e.call(0x00a0_1350, &args![target_tile, 0xfcc_u32, text, 1_u32]);
                }
            }
        }

        let opacity_setting = e.call(0x0040_3e20, &args![0x011d_9774_u32]).u32();
        let opacity = quiet_f32(e.mem.f32(opacity_setting));
        let alpha = (f64::from(opacity) * e.global::<f64>(0x0101_e568)) as f32;
        let alpha_trait = e.call(0x00a0_1860, &args![0x0107_27cc_u32]).u32();
        let action_tile = hud_field(e, 0x9c);
        hud_set_float(e, action_tile, alpha_trait, alpha);

        let controls = e.call(0x004b_71d0, &args![]).bool();
        let input = e.mem.u32(0x011d_ea0c);
        // 00777a05 / 00777ad0: `00877720` takes no stack word (plain RET); the pushed
        // 5 and 3 (or 0) are the two words of `00703be0` (RET 8).
        let binding = e.call(0x0087_7720, &args![input]).u32();
        let key = e
            .call(
                0x0070_3be0,
                &args![binding, 5_u32, if controls { 3_u32 } else { 0_u32 }],
            )
            .u32() as u8;
        if controls {
            let glyph = match key {
                8 => Some(0x11),
                9 => Some(0x12),
                10 => Some(9),
                11 => Some(10),
                12 => Some(11),
                13 => Some(12),
                14 => Some(0x10),
                15 => Some(15),
                16 => Some(13),
                17 => Some(14),
                _ => None,
            };
            if let Some(glyph) = glyph {
                let trait_id = e.call(0x00a0_1860, &args![0x0107_34d0_u32]).u32();
                let action_tile = hud_field(e, 0x9c);
                hud_trait(e, action_tile, trait_id, glyph);
            }
        } else if key < 0xee && e.mem.u32(0x011d_52f0 + u32::from(key) * 4) != 0 {
            let setting = e.mem.u32(0x011d_52f0 + u32::from(key) * 4);
            let text = e.call(0x0040_3df0, &args![setting]).u32();
            e.with_stack(0x20, |e, buffer| {
                e.call(0x0040_6d30, &args![buffer, 0x20_u32, text]);
                e.call(0x0040_6d50, &args![buffer, 0x20_u32, 0x0103_d078_u32]);
                let trait_id = e.call(0x00a0_1860, &args![0x0107_1358_u32]).u32();
                let action_tile = hud_field(e, 0x9c);
                e.call(0x00a0_1350, &args![action_tile, trait_id, buffer, 1_u32]);
            });
        } else {
            let trait_id = e.call(0x00a0_1860, &args![0x0107_1358_u32]).u32();
            let action_tile = hud_field(e, 0x9c);
            e.call(
                0x00a0_1350,
                &args![action_tile, trait_id, 0x0101_1584_u32, 1_u32],
            );
        }
        e.call(0x0040_37d0, &args![action]);
    });
}

/// PC 00777c95–0077886d: construct a temporary ItemChange, fill the item
/// summary, then position and reveal the HUD tiles. Compiler SEH is omitted.
fn set_info_item_details(
    e: &mut Engine,
    reference: Ptr,
    actor: bool,
    marked_actor: bool,
    display_mask: u32,
) -> bool {
    let is_dropped_item = e.vcall(reference.addr(), 0x224, &args![]).bool();
    let dropped = if is_dropped_item { reference.addr() } else { 0 };
    let has_nested = dropped != 0 && e.call(0x009d_9f00, &args![dropped]).u32() != 0;
    let dropped_form = if dropped != 0 {
        e.call(0x0047_6c90, &args![dropped]).u32()
    } else {
        0
    };
    let item_form_override =
        if dropped_form != 0 && e.call(0x004c_0bf0, &args![dropped_form]).bool() {
            dropped_form
        } else {
            0
        };

    e.with_stack(0xc, |e, item| {
        if has_nested {
            let nested = e.call(0x009d_9f00, &args![dropped]).u32();
            e.call(0x004b_c650, &args![item, nested]);
        } else {
            let form = if item_form_override != 0 {
                item_form_override
            } else {
                e.call(0x007a_f430, &args![reference]).u32()
            };
            e.with_stack(0xc, |e, temporary| {
                let constructed = e.call(0x004b_c550, &args![temporary, form, 0_u32]).u32();
                e.call(0x004b_c650, &args![item, constructed]);
                e.call(0x004b_c5f0, &args![temporary]);
            });
        }
        if e.mem.u32(item.addr()) == 0 {
            let storage = e.call(0x0040_1000, &args![8_u32]).u32();
            let list = if storage != 0 {
                e.call(0x0096_a2d0, &args![storage]).u32()
            } else {
                0
            };
            e.mem.set_u32(item.addr(), list);
        }
        let extra = e.call(0x005d_43c0, &args![reference]).u32();
        e.with_stack(4, |e, extra_slot| {
            e.mem.set_u32(extra_slot.addr(), extra);
            let list = e.mem.u32(item.addr());
            e.call(0x005a_e3d0, &args![list, extra_slot]);
        });

        let first_form = e.call(0x007a_f430, &args![reference]).u32();
        let first_type = e.call(0x0040_1170, &args![first_form]).u32();
        let water = if first_type == 0x15 || first_type == 0x23 {
            let form = e.call(0x007a_f430, &args![reference]).u32();
            e.vcall(form, 0x140, &args![]).u32()
        } else {
            0
        };
        let mut mask = display_mask;
        if water != 0 {
            let radiation = e.call(0x0077_8950, &args![water]).u32();
            let health = e.call(0x0077_8970, &args![water]).u32();
            e.call(0x0077_8a90, &args![health, 0x23_u32, 0x24_u32]);
            e.call(0x0077_8990, &args![radiation, 0x25_u32, 0x26_u32]);
            mask |= 0xfc;
        } else {
            let value = quiet_f32(e.call(0x004b_d400, &args![item]).f64() as f32);
            let zero: f64 = e.global(0x0101_2060);
            if f64::from(value).partial_cmp(&zero) != Some(core::cmp::Ordering::Greater) {
                e.call(0x0040_6d30, &args![0x011d_9c40_u32, 8_u32, 0x0104_7008_u32]);
            } else {
                let one: f64 = e.global(0x0101_2070);
                let format = if f64::from(value) < one {
                    0x0102_0768
                } else {
                    0x0106_ecc8
                };
                e.call(
                    0x0040_6d00,
                    &args![0x011d_9c40_u32, 8_u32, format, f64::from(value)],
                );
            }

            let form = e.call(0x0044_ddc0, &args![item]).u32();
            let form_type = e.call(0x0040_1170, &args![form]).u32();
            let mut weight = if form_type == 0x28 {
                let form = e.call(0x0044_ddc0, &args![item]).u32();
                let mod_active = e.call(0x004b_da70, &args![item, 4_u32]).bool();
                quiet_f32(e.call(0x004b_e380, &args![form, mod_active]).f64() as f32)
            } else {
                let player = current_player(e);
                let uses_weight = e.call(0x004d_1360, &args![player]).bool();
                let form = e.call(0x0044_ddc0, &args![item]).u32();
                quiet_f32(e.call(0x0048_ebc0, &args![form, uses_weight]).f64() as f32)
            };
            if form_type == 0x28 && f64::from(weight) >= e.global::<f64>(0x0102_0758) {
                e.with_stack(4, |e, factor| {
                    e.mem.set_f32(factor.addr(), 1.0);
                    let player = current_player(e);
                    e.call(0x005e_58f0, &args![0x49_u32, player, factor]);
                    weight = quiet_f32(weight * e.mem.f32(factor.addr()));
                });
            }
            let zero: f64 = e.global(0x0101_2060);
            if f64::from(weight).partial_cmp(&zero) != Some(core::cmp::Ordering::Greater) {
                e.call(0x0040_6d30, &args![0x011d_9c38_u32, 8_u32, 0x0104_7008_u32]);
            } else {
                e.call(
                    0x0040_6d00,
                    &args![0x011d_9c38_u32, 8_u32, 0x0102_0768_u32, f64::from(weight)],
                );
            }
            let weight_tile = hud_field(e, 0xb4);
            hud_set_text(e, weight_tile, 0x011d_9c38);
            let weight_label = e.call(0x0040_3df0, &args![0x011d_2aa8_u32]).u32();
            let weight_label_tile = hud_field(e, 0xb8);
            hud_set_text(e, weight_label_tile, weight_label);
            let value_tile = hud_field(e, 0xbc);
            hud_set_text(e, value_tile, 0x011d_9c40);
            let value_label = e.call(0x0040_3df0, &args![0x011d_26a0_u32]).u32();
            let value_label_tile = hud_field(e, 0xc0);
            hud_set_text(e, value_label_tile, value_label);
        }

        if e.mem.u32(0x011d_9c34) & 1 == 0 {
            let flags = e.mem.u32(0x011d_9c34);
            e.mem.set_u32(0x011d_9c34, flags | 1);
            let parent = hud_field(e, 0x138);
            let button_text = e.call(0x00a0_8b20, &args![parent, 0x0107_34c4_u32]).u32();
            e.mem.set_u32(0x011d_9c30, button_text);
        }
        let usable = e.call(0x0057_9690, &args![reference]).bool();
        let hidden = (!usable || (actor && e.call(0x0056_6950, &args![reference]).bool()))
            && (!actor || {
                let player = current_player(e);
                !e.call(0x008b_c700, &args![reference, player]).bool() && !marked_actor
            });
        let mode = if hidden { 1 } else { 2 };
        for offset in (0x9c_u32..=0xc4).step_by(4) {
            let target = hud_field(e, offset);
            hud_trait(e, target, 0xff4, mode);
        }
        let button_text = e.mem.u32(0x011d_9c30);
        e.call(0x00a0_7530, &args![button_text, 8_u32]);

        let name_tile = hud_field(e, 0xa8);
        let name_y = hud_float(e, name_tile, 0x100c);
        let name_tile = hud_field(e, 0xa8);
        let font_size = hud_float(e, name_tile, 0xfba);
        let name_tile = hud_field(e, 0xa8);
        let font_raw = e.call(0x00a0_11b0, &args![name_tile, 0xfb9_u32]).f64();
        let font = x87_to_int(e, font_raw);
        let name_tile = hud_field(e, 0xa8);
        let text = e.call(0x00a0_11f0, &args![name_tile, 0xfc4_u32]).u32();
        let measured_height = e.with_stack(12, |e, dimensions| {
            let manager = e.call(0x005b_d5b0, &args![]).u32();
            e.call(
                0x00a1_b020,
                &args![manager, dimensions, text, font, font_size, 0_u32],
            );
            e.mem.f32(dimensions.addr() + 4)
        });
        let action_tile = hud_field(e, 0x9c);
        let action_y = hud_float(e, action_tile, 0x1004);
        let shift = quiet_f32(
            (f64::from(action_y) - (f64::from(measured_height) - f64::from(name_y))) as f32,
        );
        let shift_trait = e.call(0x00a0_1860, &args![0x0106_f8f4_u32]).u32();
        let action_tile = hud_field(e, 0x9c);
        hud_set_float(e, action_tile, shift_trait, shift);
        let action_tile = hud_field(e, 0x9c);
        let action_height = e.call(0x00a0_11b0, &args![action_tile, 0xfb0_u32]).f64();
        let name_tile = hud_field(e, 0xa8);
        hud_set_float(
            e,
            name_tile,
            0xfa2,
            quiet_f32((action_height + f64::from(shift)) as f32),
        );
        let parent_tile = hud_field(e, 0x138);
        let parent_y = e.call(0x00a0_11b0, &args![parent_tile, 0xfa2_u32]).f64();
        let background_tile = hud_field(e, 0x13c);
        let background_height = e
            .call(0x00a0_11b0, &args![background_tile, 0xfb0_u32])
            .f64();
        let background_y =
            (parent_y + f64::from(shift) - background_height - e.global::<f64>(0x0101_e2c0)) as f32;
        let background_tile = hud_field(e, 0x13c);
        hud_set_float(e, background_tile, 0xfa2, quiet_f32(background_y));

        let action_tile = hud_field(e, 0x9c);
        hud_trait(e, action_tile, 0xfa3, mask & 2);
        let name_tile = hud_field(e, 0xa8);
        hud_trait(e, name_tile, 0xfa3, u32::from(mask & 4 != 0));
        if mask & 6 != 0 {
            for offset in [0x7c_u32, 0x80, 0x84] {
                let tile = hud_field(e, offset);
                hud_trait(e, tile, 0xfa3, 0);
            }
        }
        let lock_tile = hud_field(e, 0xac);
        hud_trait(e, lock_tile, 0xfa3, u32::from(mask & 4 != 0));
        let value = if mask & 4 == 0 {
            0
        } else {
            let lock_tile = hud_field(e, 0xac);
            let lock_alpha = e.call(0x00a0_11b0, &args![lock_tile, 0xfa9_u32]).f64();
            u32::from(lock_alpha == e.global::<f64>(0x0101_2060))
        };
        let container_tile = hud_field(e, 0xb0);
        hud_trait(e, container_tile, 0xfa3, value);
        for (offset, bit) in [
            (0xb4_u32, 8_u32),
            (0xb8, 0x10),
            (0xbc, 0x20),
            (0xc0, 0x40),
            (0xc4, 0x80),
        ] {
            let tile = hud_field(e, offset);
            hud_trait(e, tile, 0xfa3, u32::from(mask & bit != 0));
        }
        e.call(0x004b_c5f0, &args![item]);
    });
    true
}

fn clear_info_target_tiles(e: &mut Engine) -> bool {
    for offset in [0x9c_u32, 0xa8, 0xac, 0xb0, 0xb4, 0xb8, 0xbc, 0xc0, 0xc4] {
        let target_tile = hud_field(e, offset);
        hud_trait(e, target_tile, 0xfa3, 0);
    }
    if e.mem.u8(0x011d_96ba) == 0 {
        let target_tile = hud_field(e, 0xf4);
        hud_trait(e, target_tile, 0xff4, 1);
    }
    set_info_target_string(e);
    e.mem.set_u8(0x011d_96bb, 0);
    if e.call(0x0045_8030, &args![]).u32() == 2 {
        let target_tile = hud_field(e, 0x120);
        hud_trait(e, target_tile, 0xfa3, 1);
    }
    true
}

fn add_hud_fade(e: &mut Engine, hud_tile: u32) {
    let fade_setting = e.call(0x0040_3e20, &args![0x011d_9774_u32]).u32();
    let alpha = (f64::from(e.mem.f32(fade_setting)) * e.global::<f64>(0x0101_e568)) as f32;
    let duration: f32 = e.global(0x0101_62c0);
    e.call(
        0x00a0_7c60,
        &args![hud_tile, 0xfa9_u32, 0.0_f32, alpha, duration, 0_u32],
    );
    let now = e.call(0x0082_5c00, &args![0x011f_6394_u32]).u32();
    e.call(0x0071_5c60, &args![hud_tile, 0x1004_u32, now]);
}

// Translated from 00709430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `TESObjectWEAP::GetAmmoRegenRate` (Xbox PDB): reads the base regeneration
/// rate and optionally adds weapon-mod effect 6 before rounding to a float.
pub fn tes_object_weap_get_ammo_regen_rate(e: &mut Engine, weapon: Ptr, include_mod: bool) -> f64 {
    let base = e.mem.f32(weapon.addr() + 0x1a4);
    if include_mod {
        let modifier = e.call(0x004b_cf60, &args![weapon, 6_u32, 0_u32]).f64();
        (modifier + f64::from(base)) as f32 as f64
    } else {
        f64::from(base)
    }
}

// Translated from 0076b960 (decompiled, FalloutNV.exe 1.4.0.525)
/// Allocates a custom quest-text update and queues it according to priority.
/// Zero font IDs become -1 after the game's warning; invalid priorities
/// warn and enter the normal queue.
#[allow(clippy::too_many_arguments)] // Native ABI has seven parameters.
pub fn fn_0076b960(
    e: &mut Engine,
    title: Ptr,
    subtitle: Ptr,
    priority: i32,
    display_time: u32,
    mut title_font: i32,
    mut subtitle_font: i32,
    extra_text: Ptr,
) {
    let update = e.call(0x0040_1000, &args![0x324_u32]).u32();
    e.mem.set_u32(update + 4, 3);
    e.call(0x0040_6d30, &args![update + 8, 0x104_u32, title]);
    e.call(0x0040_6d30, &args![update + 0x10c, 0x104_u32, subtitle]);
    e.mem.set_u32(update + 0x210, display_time);
    e.mem.set_u32(update + 0x214, priority as u32);
    if title_font == 0 {
        e.call(0x005b_5e40, &args![7_u32, 0x0107_2d78_u32]);
        title_font = -1;
    }
    e.mem.set_u32(update + 0x218, title_font as u32);
    if subtitle_font == 0 {
        e.call(0x005b_5e40, &args![7_u32, 0x0107_2cf8_u32]);
        subtitle_font = -1;
    }
    e.mem.set_u32(update + 0x21c, subtitle_font as u32);
    e.call(0x0040_6d30, &args![update + 0x220, 0x104_u32, extra_text]);
    if priority == 0 {
        e.call(0x0076_baa0, &args![update]);
    } else {
        if !(1..=2).contains(&priority) {
            e.call(0x005b_5e40, &args![7_u32, 0x0107_2c88_u32, priority]);
        }
        e.call(0x0076_bad0, &args![update]);
    }
}

// Translated from 0076bdc0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::GetClass` (Xbox PDB): menu class ID 0x3ec.
pub fn hud_main_menu_get_class(_e: &mut Engine, _this: Ptr<HUDMainMenu>) -> u32 {
    0x3ec
}

// Translated from 0076bdd0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::AttachTileByID` (Xbox PDB): stores tile IDs 1 through 86.
pub fn hud_main_menu_attach_tile_by_id(
    e: &mut Engine,
    this: Ptr<HUDMainMenu>,
    id: i32,
    attached_tile: Ptr,
) {
    if (1..=0x56).contains(&id) {
        e.mem.set_u32(tile(this, id as u32), attached_tile.addr());
    }
}

// Translated from 0076be00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu` scalar deleting destructor (Xbox PDB): destroys the menu
/// and optionally frees its storage.
pub fn hud_main_menu_scalar_deleting_destructor(
    e: &mut Engine,
    this: Ptr<HUDMainMenu>,
    flags: u32,
) -> Ptr<HUDMainMenu> {
    fn_0076be30(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0076be30 (decompiled, FalloutNV.exe 1.4.0.525)
/// Destroys the HUD menu and its owned tiles, messages, arrays and base menu.
/// The compiler's SEH/unwind bookkeeping is omitted.
pub fn fn_0076be30(e: &mut Engine, this: Ptr<HUDMainMenu>) {
    e.mem.set_u32(this.addr(), 0x0107_2df4);
    let menu_tile = e.call(0x0072_6070, &args![e.mem.u32(0x011d_96c0)]).u32();
    if menu_tile != 0 {
        e.vcall(menu_tile, 0, &args![1_u32]);
    }
    let audio = e.call(0x0045_0b80, &args![1_u32]).u32();
    let fade = e.call(0x0055_9450, &args![0x011d_9770_u32]).u32();
    e.vcall(audio, 0xe8, &args![fade]);
    e.call(0x0066_b0d0, &args![0x011d_9770_u32, 0_u32]);
    e.call(0x0066_b0d0, &args![0x011d_975c_u32, 0_u32]);
    e.mem.set_u32(0x011d_96bc, 0);
    loop {
        let node = e.call(0x0068_15c0, &args![this.addr() + 0x18c]).u32();
        let message = e.mem.u32(node);
        if message == 0 {
            break;
        }
        e.call(0x0040_1030, &args![message]);
        e.call(0x0063_f7b0, &args![this.addr() + 0x18c]);
    }
    e.mem.set_u32(0x011d_96c0, 0);
    for (offset, callee) in [
        (0x26c, 0x0046_ffb0),
        (0x264, 0x0046_ffb0),
        (0x258, 0x0046_ffb0),
        (0x1c8, 0x0070_1520),
        (0x1ac, 0x0048_3710),
        (0x198, 0x0077_f9a0),
        (0x18c, 0x0046_ffb0),
    ] {
        let field = Ptr::new(this.addr() + offset);
        if callee == 0x0077_f9a0 {
            fn_0077f9a0(e, field);
        } else {
            e.call(callee, &args![field]);
        }
    }
    e.call(0x00a1_c520, &args![this]);
}

// Translated from 00770070 (decompiled, FalloutNV.exe 1.4.0.525)
/// Checks whether the HUD's tile slots 1 through 85 are populated.
pub fn fn_00770070(e: &mut Engine, this: Ptr<HUDMainMenu>) -> bool {
    (1..0x56).all(|id| e.mem.u32(tile(this, id)) != 0)
}

// Translated from 007700b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Replaces the `NiPointer` at menu offset 0x64.
pub fn fn_007700b0(e: &mut Engine, this: Ptr<HUDMainMenu>, object: Ptr) {
    e.call(0x0066_b0d0, &args![this.addr() + 0x64, object]);
}

// Translated from 007700d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a visibility flag at menu offset 0x91.
pub fn fn_007700d0(e: &mut Engine, this: Ptr<HUDMainMenu>, visible: u8) {
    e.mem.set_u8(this.addr() + 0x91, visible);
}

// Translated from 007700f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Computes a target's displayed damage threshold against the player, using
/// the game's process-acquire form and the two applicable perk entry points.
pub fn fn_007700f0(e: &mut Engine, player: Ptr, target: Ptr) -> f64 {
    if player.is_null() || target.is_null() {
        return 0.0;
    }
    let base = e.vcall(target.addr() + 0xa4, 0x0c, &args![0x4c_u32]).f32();
    let base = e.call(0x0040_4010, &args![base, 0.0_f32]).f32();
    let process = e.call(0x008d_8520, &args![player]).u32();
    let acquired = e.vcall(process, 0x148, &args![]).u32();
    let form = if acquired == 0 {
        e.mem.u32(0x011c_a278)
    } else {
        e.call(0x0044_ddc0, &args![acquired]).u32()
    };
    e.with_stack(8, |e, storage| {
        e.mem.set_f32(storage.addr(), 0.0);
        e.call(0x005e_58f0, &args![0x3a_u32, player, form, target, storage]);
        e.mem.set_f32(storage.addr() + 4, 0.0);
        e.call(
            0x005e_58f0,
            &args![0x38_u32, target, player, form, storage.addr() + 4],
        );
        let adjusted = ((f64::from(base) - f64::from(e.mem.f32(storage.addr())))
            + f64::from(e.mem.f32(storage.addr() + 4))) as f32;
        let clamped = e.call(0x0040_4010, &args![0.0_f32, adjusted]).f32();
        f64::from(clamped)
    })
}

// Translated from 007701e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::UpdateEnemyHealth` (Xbox PDB): updates the enemy meter and
/// its damage threshold text from actor values and perk entry points.
pub fn hud_main_menu_update_enemy_health(e: &mut Engine, this: Ptr<HUDMainMenu>, enemy: Ptr) {
    let actor_values = enemy.addr() + 0xa4;
    let maximum = e.vcall(actor_values, 0, &args![0x10_u32]).i32() as f32;
    let current = e.vcall(actor_values, 8, &args![0x10_u32]).i32() as f32;
    let player: Ptr = Ptr::new(e.mem.u32(0x011d_ea3c));
    let threshold = fn_007700f0(e, player, enemy) as f32;
    let zero: f64 = e.global(0x0101_2060);
    let ratio = if f64::from(maximum) > zero {
        (f64::from(current) / f64::from(maximum)) as f32
    } else {
        0.0
    };
    if e.mem.u32(0x011d_97bc) & 1 == 0 {
        let flags = e.mem.u32(0x011d_97bc);
        e.mem.set_u32(0x011d_97bc, flags | 1);
        e.mem.set_f32(0x011d_97b8, ratio);
    }
    if ratio != e.mem.f32(0x011d_97b8) {
        let timer = e.call(0x0082_5c00, &args![this.addr() + 0x224]).u32();
        e.set(this, HUDMainMenu::iEnemyHealthTimer, timer);
        e.mem.set_f32(0x011d_97b8, ratio);
    }
    if e.call(0x0047_7ba0, &args![enemy]).bool() {
        let enemy_meter = e.mem.u32(this.addr() + 0x80);
        hud_main_menu_update_meter(e, Ptr::new(enemy_meter), 0.0, 2, 0, 0);
    } else {
        let show = !e.vcall(enemy.addr(), 0x22c, &args![0_u32]).bool();
        let enemy_meter = e.mem.u32(this.addr() + 0x80);
        hud_main_menu_update_meter(e, Ptr::new(enemy_meter), ratio, 2, i32::from(show), 0);
    }
    e.with_stack(0x108, |e, storage| {
        e.mem.set_f32(storage.addr(), 0.0);
        let current_player = e.mem.u32(0x011d_ea3c);
        e.call(0x005e_58f0, &args![0x2f_u32, current_player, storage]);
        if f64::from(e.mem.f32(storage.addr())) > zero {
            let damage_label = e.call(0x0040_3df0, &args![0x011d_396c_u32]).u32();
            let threshold_as_int = e.call(0x00ec_62c0, &args![f64::from(current)]).i32();
            let health_label = e.call(0x0040_3df0, &args![0x011d_50b0_u32]).u32();
            let name = e.call(0x0055_d520, &args![enemy]).u32();
            e.call(
                0x00ec_623a,
                &args![
                    storage.addr() + 4,
                    0x0107_3460_u32,
                    name,
                    health_label,
                    threshold_as_int,
                    damage_label,
                    f64::from(threshold)
                ],
            );
            let current_menu = e.mem.u32(0x011d_96c0);
            let hud_text = e.mem.u32(current_menu + 0x84);
            e.call(
                0x00a0_1350,
                &args![hud_text, 0xfc4_u32, storage.addr() + 4, 1_u32],
            );
        } else {
            let name = e.call(0x0055_d520, &args![enemy]).u32();
            let current_menu = e.mem.u32(0x011d_96c0);
            let hud_text = e.mem.u32(current_menu + 0x84);
            e.call(0x00a0_1350, &args![hud_text, 0xfc4_u32, name, 1_u32]);
        }
    });
}

fn current_hud_tile(e: &Engine, offset: u32) -> u32 {
    e.mem.u32(e.mem.u32(0x011d_96c0) + offset)
}

fn hud_visible(e: &mut Engine, offset: u32) -> bool {
    let current_tile = current_hud_tile(e, offset);
    e.call(0x00a0_1230, &args![current_tile, 0xfa3_u32]).u8() != 0
}

fn fade_finished(e: &mut Engine, offset: u32) -> bool {
    let current_tile = current_hud_tile(e, offset);
    e.call(0x00a0_7fc0, &args![current_tile, 0xfa9_u32]).u8() != 0
}

fn hud_alpha(e: &mut Engine, offset: u32) -> f64 {
    let current_tile = current_hud_tile(e, offset);
    e.call(0x00a0_11b0, &args![current_tile, 0xfa9_u32]).f64()
}

fn hud_time(e: &mut Engine, object: u32) -> u32 {
    e.call(0x0082_5c00, &args![object]).u32()
}

fn quiet_f32(value: f32) -> f32 {
    let bits = value.to_bits();
    f32::from_bits(
        if bits & 0x7f80_0000 == 0x7f80_0000 && bits & 0x007f_ffff != 0 {
            bits | 0x0040_0000
        } else {
            bits
        },
    )
}

fn fistp_i64_low(value: f64) -> u32 {
    let converted = if value.is_finite()
        && (-9_223_372_036_854_775_808.0..9_223_372_036_854_775_808.0).contains(&value)
    {
        value.trunc() as i64
    } else {
        i64::MIN
    };
    converted as u32
}

fn do_idle_geometry(e: &mut Engine, this: Ptr<HUDMainMenu>) {
    let width = e.call(0x0071_5da0, &args![]).f64();
    let width = x87_to_int(e, width);
    let screen_width = e.call(0x0071_5d40, &args![]).f64();
    let screen_width = x87_to_int(e, screen_width);
    let compass_width = e.call(0x0071_77c0, &args![]).f64();
    let compass_width = x87_to_int(e, compass_width);
    let inset = e.call(0x0071_7820, &args![]).f64();
    let inset = x87_to_int(e, inset);
    let current_tile = current_hud_tile(e, 0x118);
    hud_trait(
        e,
        current_tile,
        0xfa1,
        compass_width.wrapping_mul(2).wrapping_add(10) as u32,
    );
    let current_tile = current_hud_tile(e, 0x118);
    let current_width = e.call(0x00a0_11b0, &args![current_tile, 0xfb0_u32]).f64();
    let new_width = (f64::from(width.wrapping_sub(inset.wrapping_mul(2))) - current_width) as f32;
    let current_tile = current_hud_tile(e, 0x118);
    hud_set_float(e, current_tile, 0xfa2, new_width);
    let current_tile = current_hud_tile(e, 0x114);
    let current_x = e.call(0x00a0_11b0, &args![current_tile, 0xfb1_u32]).f64();
    let horizontal_adjustment: f64 = e.global(0x0101_db88);
    let new_x = (f64::from(screen_width.wrapping_sub(compass_width.wrapping_mul(2))) - current_x
        + horizontal_adjustment) as f32;
    let current_tile = current_hud_tile(e, 0x114);
    hud_set_float(e, current_tile, 0xfa1, new_x);
    let current_tile = current_hud_tile(e, 0x114);
    let current_width = e.call(0x00a0_11b0, &args![current_tile, 0xfb0_u32]).f64();
    let new_width = (f64::from(width.wrapping_sub(inset.wrapping_mul(2))) - current_width) as f32;
    let current_tile = current_hud_tile(e, 0x114);
    hud_set_float(e, current_tile, 0xfa2, new_width);
    for offset in [0x118, 0x114] {
        let current_tile = current_hud_tile(e, offset);
        e.call(0x00a0_7530, &args![current_tile, 1_u32]);
    }
    hud_main_menu_update_actor_value(e, this, 0x10);
    hud_main_menu_update_actor_value(e, this, 0x0c);
    e.call(0x0077_e4f0, &args![this]);
    for offset in [0x180, 0x160, 0x154, 0x170, 0x174, 0x158, 0x15c] {
        let alpha = quiet_f32(e.mem.f32(0x011d_979c));
        let current_tile = current_hud_tile(e, offset);
        hud_set_float(e, current_tile, 0xfa9, alpha);
    }
}

fn do_idle_fades(e: &mut Engine) {
    for (offset, from_setting) in [(0x64_u32, false), (0x68, true)] {
        if hud_alpha(e, offset) > e.global::<f64>(0x0101_2060) && fade_finished(e, offset) {
            let now = hud_time(e, 0x011f_6394);
            let current_tile = current_hud_tile(e, offset);
            let start = e.call(0x00a0_11b0, &args![current_tile, 0x1004_u32]).f64();
            let timeout: f64 = e.global(0x0107_3480);
            if f64::from(now) - start > timeout {
                let alpha = if from_setting {
                    let setting = e.call(0x0040_3e20, &args![0x011d_9774_u32]).u32();
                    (f64::from(e.mem.f32(setting)) * e.global::<f64>(0x0101_e568)) as f32
                } else {
                    hud_alpha(e, offset) as f32
                };
                let duration: f32 = quiet_f32(e.global(0x0101_62c0));
                let current_tile = current_hud_tile(e, offset);
                e.call(
                    0x00a0_7c60,
                    &args![current_tile, 0xfa9_u32, alpha, 0.0_f32, duration, 0_u32],
                );
            }
        }
    }
    let hold_trait = e.call(0x00a0_1860, &args![0x0107_3474_u32]).u32();
    let text_tile = current_hud_tile(e, 0x64);
    if e.call(0x00a0_1230, &args![text_tile, hold_trait]).u8() != 0 {
        let compass = current_hud_tile(e, 0x128);
        if e.call(0x00a0_11b0, &args![compass, 0xfa3_u32]).f64() != e.global::<f64>(0x0101_2060) {
            let subtitle = current_hud_tile(e, 0x68);
            if e.call(0x00a0_11b0, &args![subtitle, 0xfa9_u32]).f64()
                == e.global::<f64>(0x0101_2060)
                && fade_finished(e, 0x68)
            {
                let duration: f32 = quiet_f32(e.global(0x0101_62c0));
                let setting = e.call(0x0040_3e20, &args![0x011d_9774_u32]).u32();
                let alpha = (f64::from(e.mem.f32(setting)) * e.global::<f64>(0x0101_e568)) as f32;
                let current_tile = current_hud_tile(e, 0x64);
                e.call(
                    0x00a0_7c60,
                    &args![current_tile, 0xfa9_u32, 0.0_f32, alpha, duration, 0_u32],
                );
                let now = hud_time(e, 0x011f_6394);
                let current_tile = current_hud_tile(e, 0x64);
                e.call(0x0071_5c60, &args![current_tile, 0x1004_u32, now]);
                let hold_trait = e.call(0x00a0_1860, &args![0x0107_3474_u32]).u32();
                let current_tile = current_hud_tile(e, 0x64);
                hud_trait(e, current_tile, hold_trait, 0);
            }
        }
    }
    let player = e.mem.u32(0x011d_ea3c);
    if e.call(0x0049_97b0, &args![player]).u8() != 0 {
        let menu = e.mem.u32(0x011d_96c0);
        hud_main_menu_update_sneak(e, Ptr::new(menu));
    } else if hud_alpha(e, 0x88) > e.global::<f64>(0x0101_2060) && fade_finished(e, 0x88) {
        let duration: f32 = quiet_f32(e.global(0x0101_6248));
        let alpha = hud_alpha(e, 0x88) as f32;
        let current_tile = current_hud_tile(e, 0x88);
        e.call(
            0x00a0_7c60,
            &args![current_tile, 0xfa9_u32, alpha, 0.0_f32, duration, 0_u32],
        );
    }
}

fn do_idle_target(e: &mut Engine, this: Ptr<HUDMainMenu>) {
    e.mem.set_u8(0x011d_96c5, 0);
    if e.mem.u32(this.addr() + 0x21c) != 0 {
        let player = e.mem.u32(0x011d_ea3c);
        let mut entry = e.call(0x0059_5d10, &args![player]).u32();
        while entry != 0 && e.call(0x0082_56d0, &args![entry]).u8() == 0 {
            let first = e.call(0x0068_15c0, &args![entry]).u32();
            let object = e.mem.u32(first);
            if e.mem.u32(object) == e.mem.u32(this.addr() + 0x21c)
                && e.mem.u8(object + 4) != 0
                && e.mem.u8(object + 5) == 0
            {
                e.mem.set_u8(0x011d_96c5, 1);
                break;
            }
            entry = e.call(0x0072_6070, &args![entry]).u32();
        }
    }
    let player = e.mem.u32(0x011d_ea3c);
    let heading = e.vcall(player, 0x2bc, &args![0_u32]).f64();
    let degrees_per_radian: f64 = e.global(0x0102_f248);
    let heading = (heading * degrees_per_radian) as f32;
    let mut target_heading = 0.0_f32;
    let target = e.mem.u32(this.addr() + 0x21c);
    if target != 0 {
        let target_location = e.vcall(target, 0x1f4, &args![]).u32();
        let player = e.mem.u32(0x011d_ea3c);
        let player_location = e.vcall(player, 0x1f4, &args![]).u32();
        target_heading = hud_main_menu_heading_between_locations(
            e,
            Ptr::new(player_location),
            Ptr::new(target_location),
        ) as f32;
    }
    let now = hud_time(e, this.addr() + 0x224);
    let elapsed = now.wrapping_sub(e.mem.u32(this.addr() + 0x244));
    let timing_setting = e.call(0x0040_3e20, &args![0x011c_f87c_u32]).u32();
    let timeout =
        fistp_i64_low(f64::from(e.mem.f32(timing_setting)) * e.global::<f64>(0x0101_7b70));
    let close_enough = if elapsed <= timeout {
        true
    } else {
        let difference = (f64::from(heading) - f64::from(target_heading)) as f32;
        let angle = e.call(0x0040_8840, &args![difference]).f64();
        angle < e.global::<f64>(0x0103_57e8)
    };
    let mut show_enemy = false;
    if e.call(0x0044_ddc0, &args![0x011f_2250_u32]).u32() == 4 {
        let actor = e.mem.u32(0x011f_21cc);
        if actor != 0 && e.vcall(actor, 0x100, &args![]).u8() != 0 {
            let actor = e.mem.u32(0x011f_21cc);
            if e.vcall(actor, 0x22c, &args![0_u32]).u8() == 0 {
                show_enemy = true;
                let actor = e.mem.u32(0x011f_21cc);
                hud_main_menu_set_enemy_actor(e, Ptr::new(actor));
            }
        }
    } else {
        let target = e.mem.u32(this.addr() + 0x21c);
        let mut show_ref = false;
        if target != 0 && e.vcall(target, 0x22c, &args![0_u32]).u8() == 0 {
            let target = e.mem.u32(this.addr() + 0x21c);
            if e.call(0x0047_7ba0, &args![target]).u8() == 0
                && e.mem.u8(0x011d_96c5) != 0
                && !hud_visible(e, 0x9c)
                && !hud_visible(e, 0xa8)
            {
                let current_tile = current_hud_tile(e, 0x120);
                let state = e.call(0x00a0_11b0, &args![current_tile, 0xff4_u32]).f64();
                if state == e.global::<f64>(0x0101_1590) && close_enough {
                    show_ref = true;
                }
            }
        }
        if show_ref {
            show_enemy = true;
            let last = e.mem.u32(this.addr() + 0x1b8);
            e.call(0x0077_5a00, &args![last, 0_u32, 1_u32]);
        } else {
            let current_tile = current_hud_tile(e, 0x120);
            let state = e.call(0x00a0_11b0, &args![current_tile, 0xff4_u32]).f64();
            if state == e.global::<f64>(0x0101_1590) {
                for offset in [0x7c, 0x80, 0x84] {
                    let current_tile = e.mem.u32(this.addr() + offset);
                    hud_trait(e, current_tile, 0xfa3, 0);
                }
            }
        }
    }
    if show_enemy {
        let mut perceived = false;
        let target = e.mem.u32(this.addr() + 0x21c);
        if target != 0 {
            let player = e.mem.u32(0x011d_ea3c);
            let actor = e.call(0x0096_7090, &args![player, target]).u32();
            if actor != 0 {
                let player = e.mem.u32(0x011d_ea3c);
                let target = e.mem.u32(this.addr() + 0x21c);
                let actor = e.call(0x0096_7090, &args![player, target]).u32();
                perceived = e.mem.u8(actor + 4) != 0;
            }
            if !perceived {
                let target = e.mem.u32(this.addr() + 0x21c);
                let player = e.mem.u32(0x011d_ea3c);
                perceived = e.call(0x008b_c700, &args![target, player]).u8() != 0;
            }
        }
        for offset in [0x7c, 0x80, 0x84] {
            let current_tile = e.mem.u32(this.addr() + offset);
            hud_trait(e, current_tile, 0xfa3, 1);
        }
        let current_tile = current_hud_tile(e, 0x120);
        hud_trait(e, current_tile, 0xff4, u32::from(perceived) + 1);
        for offset in [0x7c, 0x80, 0x84, 0x84] {
            let current_tile = current_hud_tile(e, offset);
            e.call(0x00a0_7530, &args![current_tile, 8_u32]);
        }
    }
    let target = e.mem.u32(this.addr() + 0x21c);
    if target != 0 {
        let qualifies = e.call(0x0049_3bb0, &args![target]).u8() != 0
            || e.call(0x0044_ddc0, &args![0x011f_2250_u32]).u32() == 4;
        if qualifies {
            let target = e.mem.u32(this.addr() + 0x21c);
            hud_main_menu_update_enemy_health(e, this, Ptr::new(target));
        }
    }
    let last = e.mem.u32(this.addr() + 0x1b8);
    if last != 0 {
        let previous = e.mem.u8(0x011d_97c0);
        let current = e.call(0x0057_9690, &args![last]).u8();
        if previous != current {
            let last = e.mem.u32(this.addr() + 0x1b8);
            let current = e.call(0x0057_9690, &args![last]).u8();
            e.mem.set_u8(0x011d_97c0, current);
            let last = e.mem.u32(this.addr() + 0x1b8);
            e.call(0x0077_5a00, &args![last, 0_u32, 1_u32]);
        }
    }
}

fn do_idle_quests(e: &mut Engine, this: Ptr<HUDMainMenu>) {
    let mut display_reminder = false;
    if e.mem.u8(this.addr() + 0x205) != 0 {
        let now = hud_time(e, 0x011f_6394);
        let elapsed = now.wrapping_sub(e.mem.u32(this.addr() + 0x248));
        let setting = e.call(0x0043_d4d0, &args![0x011d_2220_u32]).u32();
        let delay = e.mem.u32(setting).wrapping_mul(1000);
        if elapsed > delay {
            let mode = e.mem.u32(e.mem.u32(0x011d_96c0) + 0x1c4);
            if mode != 0x12 && mode != 0x13 {
                display_reminder = true;
            }
        }
    }
    if display_reminder {
        let now = hud_time(e, 0x011f_6394);
        e.mem.set_u32(this.addr() + 0x248, now);
        let player = e.mem.u32(0x011d_ea3c);
        let quest = e.call(0x005c_bb50, &args![player]).u32();
        hud_main_menu_display_quest_reminder(e, Ptr::new(quest));
    } else if e.mem.u32(this.addr() + 0x248) == 0 && e.call(0x0045_8030, &args![]).u32() == 2 {
        let now = hud_time(e, 0x011f_6394);
        e.mem.set_u32(this.addr() + 0x248, now);
        let player = e.mem.u32(0x011d_ea3c);
        let quest = e.call(0x005c_bb50, &args![player]).u32();
        hud_main_menu_display_quest_reminder(e, Ptr::new(quest));
        e.mem.set_u8(this.addr() + 0x254, 0);
    }
    if e.mem.u8(this.addr() + 0x254) != 0 {
        e.call(0x0077_f500, &args![this]);
        hud_main_menu_kill_quest_objectives(e, this);
        e.mem.set_u8(this.addr() + 0x254, 0);
        return;
    }
    if e.call(0x0076_b610, &args![this.addr() + 0x198]).u8() != 0 {
        return;
    }
    let mut index = 0_u32;
    loop {
        let count = e.call(0x0044_ddc0, &args![this.addr() + 0x198]).u32();
        if index >= count {
            break;
        }
        let now = hud_time(e, this.addr() + 0x224);
        let entry = e
            .call(0x0087_7a30, &args![this.addr() + 0x198, index])
            .u32();
        let message = e.mem.u32(entry);
        let elapsed = now.wrapping_sub(e.mem.u32(message + 0x104));
        let entry = e
            .call(0x0087_7a30, &args![this.addr() + 0x198, index])
            .u32();
        let message = e.mem.u32(entry);
        let duration = f64::from(e.mem.f32(message + 0x108)) * e.global::<f64>(0x0101_7b70);
        if duration <= f64::from(elapsed) {
            let entry = e
                .call(0x0087_7a30, &args![this.addr() + 0x198, index])
                .u32();
            if e.mem.u32(entry) == e.mem.u32(this.addr() + 0x1a8) {
                e.call(0x0077_5670, &args![this]);
            }
            let entry = e
                .call(0x0087_7a30, &args![this.addr() + 0x198, index])
                .u32();
            let message = e.mem.u32(entry);
            e.call(0x0040_1030, &args![message]);
            e.call(0x006b_f8f0, &args![this.addr() + 0x198, index, 1_u32]);
        } else {
            index = index.wrapping_add(1);
        }
    }
    if e.mem.u32(this.addr() + 0x1a8) == 0 {
        let text = e.call(0x0077_5230, &args![this]).u32();
        e.mem.set_u32(this.addr() + 0x1a8, text);
        if e.mem.u32(this.addr() + 0x1a8) != 0 {
            let text = e.mem.u32(this.addr() + 0x1a8);
            let current_tile = current_hud_tile(e, 0x98);
            hud_set_text(e, current_tile, text);
            let current_tile = current_hud_tile(e, 0x98);
            hud_trait(e, current_tile, 0xfa3, 1);
        }
    }
}

fn do_idle_messages(e: &mut Engine, this: Ptr<HUDMainMenu>) {
    if e.call(0x005a_e380, &args![this.addr() + 0x18c]).u32() == 0 || !hud_visible(e, 0x134) {
        return;
    }
    if hud_alpha(e, 0x94) == e.global::<f64>(0x0101_2060) {
        for offset in [0x94, 0x8c, 0x90] {
            let alpha = quiet_f32(e.mem.f32(0x011d_979c));
            let duration: f32 = quiet_f32(e.global(0x0102_1f44));
            let current_tile = current_hud_tile(e, offset);
            e.call(
                0x00a0_7c60,
                &args![current_tile, 0xfa9_u32, 0.0_f32, alpha, duration, 0_u32],
            );
        }
    }
    if fade_finished(e, 0x94) {
        let first = e.call(0x0068_15c0, &args![this.addr() + 0x18c]).u32();
        let message = e.mem.u32(first);
        if e.mem.u32(this.addr() + 0x194) == 0 {
            let now = hud_time(e, this.addr() + 0x224);
            e.mem.set_u32(this.addr() + 0x194, now);
        }
        let now = hud_time(e, this.addr() + 0x224);
        let elapsed = now.wrapping_sub(e.mem.u32(this.addr() + 0x194));
        let duration = f64::from(e.mem.f32(message + 0x30c)) * e.global::<f64>(0x0101_7b70);
        if duration <= f64::from(elapsed) {
            e.call(0x0077_56e0, &args![this]);
        }
    }
}

// Translated from 00770430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::DoIdle` (Xbox PDB): advances the visible HUD, target,
/// reminder and message state on the game's timed UI cadence.
// Translation incomplete: 00770430 - the compiler's SEH unwind frame is omitted.
pub fn hud_main_menu_do_idle(e: &mut Engine, this: Ptr<HUDMainMenu>) {
    let player = e.mem.u32(0x011d_ea3c);
    if player == 0 || e.call(0x008d_6f30, &args![player]).u32() == 0 {
        return;
    }
    fn_00771520(e, Ptr::new(this.addr() + 0x224));
    let scene = e.mem.u32(this.addr() + 0x40);
    let a = e.mem.u32(this.addr() + 0x34);
    let b = e.mem.u32(this.addr() + 0x38);
    let c = e.mem.u32(this.addr() + 0x3c);
    e.call(0x0077_9070, &args![scene, a, b, c]);
    e.call(0x0077_da60, &args![this]);
    fn_007721c0(e, this);
    e.call(0x0077_e750, &args![this]);
    fn_0077f700(e, this);
    if e.mem.u32(0x011d_97c8) & 1 == 0 {
        let flags = e.mem.u32(0x011d_97c8);
        e.mem.set_u32(0x011d_97c8, flags | 1);
        let now = hud_time(e, this.addr() + 0x224);
        e.mem.set_u32(0x011d_97c4, now);
    }
    let now = hud_time(e, this.addr() + 0x224);
    if now.wrapping_sub(e.mem.u32(0x011d_97c4)) < 200 {
        return;
    }
    for (timer_offset, first_tile, second_tile) in [(0x24c, 0x154, 0x170), (0x250, 0x158, 0x174)] {
        let timer = e.mem.u32(this.addr() + timer_offset);
        let expired = timer == u32::MAX || hud_time(e, 0x011f_6394).wrapping_sub(timer) > 1000;
        if expired {
            e.mem.set_u32(this.addr() + timer_offset, u32::MAX);
            for offset in [first_tile, second_tile] {
                let current_tile = current_hud_tile(e, offset);
                hud_trait(e, current_tile, 0xfa3, 0);
            }
        }
    }
    let now = hud_time(e, this.addr() + 0x224);
    e.mem.set_u32(0x011d_97c4, now);
    do_idle_geometry(e, this);
    do_idle_fades(e);
    let menu = e.mem.u32(0x011d_96c0);
    if !matches!(e.mem.u32(menu + 0x1c4), 0x12 | 0x13) {
        e.call(0x0077_a650, &args![this]);
    }
    e.call(0x0077_c4e0, &args![this]);
    hud_main_menu_update_radiation_meter(e, this);
    let player = e.mem.u32(0x011d_ea3c);
    let status = e.call(0x004d_1360, &args![player]).u8() != 0;
    hud_main_menu_update_status_mode_ui(e, this, status);
    do_idle_target(e, this);
    do_idle_quests(e, this);
    do_idle_messages(e, this);
    let setting = e.call(0x0040_3e20, &args![0x011d_9774_u32]).u32();
    let alpha = (f64::from(e.mem.f32(setting)) * e.global::<f64>(0x0101_e568)) as f32;
    e.mem.set_f32(0x011d_979c, alpha);
}

// Translated from 00771520 (decompiled, FalloutNV.exe 1.4.0.525)
/// Converts a scene object and passes the result to its attached controller.
pub fn fn_00771520(e: &mut Engine, object: Ptr) {
    // 00457fe0 (the tick count) takes no `this`; 00771527 leaves ECX as is.
    let converted = e.call(0x0045_7fe0, &args![]).u32();
    e.call(0x00aa_4ee0, &args![object, converted]);
}

// Translated from 00771540 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::UpdateActorValue` (Xbox PDB): refreshes the action-point
/// or hit-point meter for actor values 12 and 16.
pub fn hud_main_menu_update_actor_value(e: &mut Engine, this: Ptr<HUDMainMenu>, value: u32) {
    let player = e.mem.u32(0x011d_ea3c);
    let actor_values = player + 0xa4;
    match value {
        0x0c => {
            let maximum = e.vcall(actor_values, 0x20, &args![value]).f32();
            let maximum = e.call(0x0040_4010, &args![0.0_f32, maximum]).f32();
            let ratio = if maximum != 0.0 {
                let current = e.vcall(actor_values, 0x0c, &args![value]).f32();
                let current = e.call(0x0040_4010, &args![0.0_f32, current]).f32();
                (f64::from(current) / f64::from(maximum)) as f32
            } else {
                0.0
            };
            let ratio = e.call(0x0040_ebd0, &args![ratio, 1.0_f32]).f32();
            let meter = e.mem.u32(tile(this, 7));
            hud_main_menu_update_meter(e, Ptr::new(meter), ratio, 1, 0, 0);
        }
        0x10 => {
            let maximum = e.vcall(actor_values, 0x20, &args![value]).f32();
            let mut ratio = if maximum != 0.0 {
                let current = e.vcall(actor_values, 0x0c, &args![value]).f32();
                let current = e.call(0x0040_4010, &args![0.0_f32, current]).f32();
                (f64::from(current) / f64::from(maximum)) as f32
            } else {
                0.0
            };
            if ratio > 1.0 {
                ratio = 1.0;
            }
            let should_animate = !e.vcall(player, 0x22c, &args![0_u32]).bool();
            let menu = e.mem.u32(0x011d_96c0);
            let meter = e.mem.u32(menu + 0x2c);
            hud_main_menu_update_meter(e, Ptr::new(meter), ratio, 0, i32::from(should_animate), 0);
        }
        _ => {}
    }
}

fn set_visibility_trait(e: &mut Engine, offset: u32, mask: u32) {
    let menu = e.mem.u32(0x011d_96c0);
    let value = e.mem.u32(menu + 0x1bc) & mask;
    let current_tile = e.mem.u32(e.mem.u32(0x011d_96c0) + offset);
    hud_trait(e, current_tile, 0xfa3, value);
}

// Translated from 00771700 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::SetMenuMode` (Xbox PDB): selects the HUD visibility mask for
/// the current menu and player state, updates each tile, and controls its sound.
pub fn hud_main_menu_set_menu_mode(e: &mut Engine, requested_mode: i32) {
    if e.mem.u32(0x011d_96c0) == 0 {
        return;
    }
    let mut mode = requested_mode;
    let menu = e.mem.u32(0x011d_96c0);
    if e.call(0x0059_bb30, &args![menu]).i32() != 1 {
        let menu = e.mem.u32(0x011d_96c0);
        e.call(0x00a1_db20, &args![menu]);
    }
    if mode == 1 {
        let top_menu = e.call(0x0070_23c0, &args![]).u32();
        mode = match top_menu {
            0 | 0x3ec => {
                let player = e.mem.u32(0x011d_ea3c);
                if e.vcall(player, 0x214, &args![]).u32() == 4 {
                    0xb
                } else {
                    let player = e.mem.u32(0x011d_ea3c);
                    if e.vcall(player, 0x214, &args![]).u32() == 9 {
                        0xb
                    } else {
                        let player = e.mem.u32(0x011d_ea3c);
                        if e.vcall(player, 0x214, &args![]).u32() != 0 {
                            10
                        } else if e.call(0x0044_ddc0, &args![0x011f_2250_u32]).u32() == 4 {
                            8
                        } else {
                            let player = e.mem.u32(0x011d_ea3c);
                            if e.call(0x005a_03f0, &args![player, 1_u32]).u8() != 0 {
                                hud_main_menu_set_menu_mode(e, 0xc);
                                1
                            } else {
                                let player = e.mem.u32(0x011d_ea3c);
                                if e.call(0x008b_bc10, &args![player]).u8() != 0 {
                                    0x16
                                } else {
                                    2
                                }
                            }
                        }
                    }
                }
            }
            0x3ea | 0x3eb | 0x3f6 | 0x3ff | 0x40b | 0x425 => 3,
            0x3ef => 5,
            0x3f0 => 9,
            0x3f1 => 6,
            0x3f4 => 0xb,
            0x3f5 => 4,
            0x41f | 0x421 => 0x10,
            0x420 => 7,
            _ => 1,
        };
    }
    if mode == 0x16 {
        let weapon = e.call(0x0055_9450, &args![0x011d_9770_u32]).u32();
        if weapon != 0 {
            let weapon = e.call(0x0055_9450, &args![0x011d_9770_u32]).u32();
            if e.call(0x0045_bc00, &args![weapon, 0_u32]).u32() != 0 {
                let player = e.mem.u32(0x011d_ea3c);
                if e.call(0x008a_16d0, &args![player]).u8() != 0 {
                    mode = 0x17;
                }
            }
        }
    }
    if mode != 0x18 && mode != 4 && mode != 5 {
        let player = e.mem.u32(0x011d_ea3c);
        if e.call(0x005a_03f0, &args![player, 1_u32]).u8() != 0 {
            let selected = e.call(0x004b_7210, &args![]).u32();
            if e.call(0x0071_9ae0, &args![selected]).u8() != 0 {
                mode = 2;
            } else if mode != 6 {
                mode = 0xc;
            }
        }
    }
    let flags = match mode {
        2 | 0xd | 0x14 | 0x16 => {
            let player = e.mem.u32(0x011d_ea3c);
            let flags = if e.call(0x005a_03f0, &args![player, 0x20_u32]).u8() != 0 {
                0x1fdff
            } else {
                0x1ffff
            };
            let menu = e.mem.u32(0x011d_96c0);
            e.mem.set_u32(menu + 0x1bc, flags);
            e.call(0x0077_8b10, &args![]);
            None
        }
        3 | 7 => Some(0x100),
        4 | 5 | 0xe | 0x11 | 0x12 | 0x15 | 0x18 | 0x13 | 0x19 => Some(0),
        6 => Some(0x1110),
        8 => Some(0x108),
        9 => Some(0x1000),
        10 => Some(0x11dbf),
        0xb => Some(0x11fff),
        0xc => Some(0x1514),
        0xf | 0x10 => Some(0x1010),
        0x17 => Some(8),
        _ => None,
    };
    if let Some(flags) = flags {
        if !matches!(mode, 10..=0xc) {
            let menu = e.mem.u32(0x011d_96c0);
            e.call(0x0077_5670, &args![menu]);
        }
        let menu = e.mem.u32(0x011d_96c0);
        e.mem.set_u32(menu + 0x1bc, flags);
    }
    let menu = e.mem.u32(0x011d_96c0);
    let flags = e.mem.u32(menu + 0x1bc) & e.mem.u32(menu + 0x1c0);
    e.mem.set_u32(menu + 0x1bc, flags);
    if e.mem.u8(0x011d_96bb) != 0 {
        let menu = e.mem.u32(0x011d_96c0);
        let flags = e.mem.u32(menu + 0x1bc);
        e.mem.set_u32(menu + 0x1bc, flags & !8);
    }
    let player = e.mem.u32(0x011d_ea3c);
    if player != 0 && fn_007721a0(e, Ptr::new(player)).addr() != 0 {
        let player = e.mem.u32(0x011d_ea3c);
        if e.call(0x0052_4d10, &args![player]).u8() == 0 {
            let menu = e.mem.u32(0x011d_96c0);
            let flags = e.mem.u32(menu + 0x1bc);
            e.mem.set_u32(menu + 0x1bc, flags & !0x40);
        }
    }
    let meter_tile = hud_tile(e, 0x47);
    let value = e.call(0x00a0_11b0, &args![meter_tile, 0x1007_u32]).f64() as f32;
    let value = if mode == 6 {
        (f64::from(value) - e.global::<f64>(0x0107_3488)) as f32
    } else {
        value
    };
    let meter_tile = hud_tile(e, 0x47);
    hud_set_float(e, meter_tile, 0xfa2, value);
    let menu = e.mem.u32(0x011d_96c0);
    let prior_mode = e.mem.u32(menu + 0x1c4);
    if prior_mode != 0x17 && mode == 0x17 {
        e.call(0x0077_f3c0, &args![1_u32]);
    } else if prior_mode == 0x17 && mode != 0x17 {
        e.call(0x0077_f3c0, &args![0_u32]);
    }
    let menu = e.mem.u32(0x011d_96c0);
    e.mem.set_u32(menu + 0x1c4, mode as u32);
    if !matches!(
        mode,
        0x14 | 6 | 2 | 5 | 0xe | 0x11 | 9 | 0xb | 0x12 | 0x13 | 0x19
    ) {
        let menu = e.mem.u32(0x011d_96c0);
        e.mem.set_u8(menu + 0x254, 1);
    }
    for (offset, bit) in [
        (0x114, 1),
        (0x118, 2),
        (0x11c, 4),
        (0x120, 8),
        (0x124, 0x10),
        (0x128, 0x20),
        (0x12c, 0x40),
        (0x130, 0x80),
        (0x134, 0x100),
        (0x138, 0x200),
        (0x13c, 0x400),
        (0x140, 0x800),
        (0x144, 0x1000),
        (0x148, 0x2000),
        (0x14c, 0x4000),
        (0x150, 0x8000),
        (0x15c, 1),
        (0x160, 0x10000),
        (0x180, 1),
    ] {
        set_visibility_trait(e, offset, bit);
    }
    let menu = e.mem.u32(0x011d_96c0);
    if e.mem.u32(menu + 0x1bc) & 1 == 0 {
        for offset in [0x154, 0x170] {
            let menu = e.mem.u32(0x011d_96c0);
            let current_tile = e.mem.u32(menu + offset);
            hud_trait(e, current_tile, 0xfa3, 0);
        }
    }
    let menu = e.mem.u32(0x011d_96c0);
    if e.mem.u32(menu + 0x1bc) & 8 == 0 {
        for offset in [0x158, 0x174] {
            let menu = e.mem.u32(0x011d_96c0);
            let current_tile = e.mem.u32(menu + offset);
            hud_trait(e, current_tile, 0xfa3, 0);
        }
    }
    let menu = e.mem.u32(0x011d_96c0);
    if e.call(0x00ad_8ce0, &args![menu + 0x1ac]).u8() != 0 {
        let menu = e.mem.u32(0x011d_96c0);
        if e.mem.u32(menu + 0x1bc) & 4 != 0 {
            let menu = e.mem.u32(0x011d_96c0);
            e.call(0x00ad_8830, &args![menu + 0x1ac, 0_u32]);
        } else {
            let menu = e.mem.u32(0x011d_96c0);
            e.call(0x00ad_88b0, &args![menu + 0x1ac]);
        }
    }
}

// Translated from 007721a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the object's pointer at PC offset 0xe34.
pub fn fn_007721a0(e: &mut Engine, object: Ptr) -> Ptr {
    Ptr::new(e.mem.u32(object.addr() + 0xe34))
}

// Translated from 00772820 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the value held by the object's inline pointer at +0xcc.
pub fn fn_00772820(e: &mut Engine, object: Ptr) -> Ptr {
    e.call(0x0055_9450, &args![object.addr() + 0xcc]).ptr()
}

// Translated from 00772840 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::HeadingBetweenLocations` (Xbox PDB): asks the source
/// location for a relative 2D vector, resolves its angle, clamps it, and
/// converts radians to degrees.
pub fn hud_main_menu_heading_between_locations(e: &mut Engine, from: Ptr, to: Ptr) -> f64 {
    e.with_stack(12, |e, relative| {
        e.call(0x0043_9ef0, &args![to, relative, from]);
        let x = e.mem.f32(relative.addr());
        let y = e.mem.f32(relative.addr() + 4);
        let mut radians = 0.0_f32;
        if y != 0.0 {
            let ratio = (f64::from(x) / f64::from(y)) as f32;
            radians = e.call(0x004b_1460, &args![ratio]).f32();
            if y < 0.0 {
                let pi: f64 = e.global(0x0101_ff40);
                radians = (f64::from(radians) + pi) as f32;
            }
        } else if x < 0.0 {
            radians = e.global(0x0102_b3c8);
        }
        let clamped = e.call(0x004b_1480, &args![radians]).f32();
        let degrees_per_radian: f64 = e.global(0x0102_f248);
        f64::from((f64::from(clamped) * degrees_per_radian) as f32)
    })
}

// Translated from 007728f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::SetRegionText` (Xbox PDB): updates the region/location
/// text when its tile is clear, with a fade when it replaces prior text.
pub fn hud_main_menu_set_region_text(e: &mut Engine, region: Ptr) {
    if e.mem.u32(0x011d_96c0) == 0 {
        return;
    }
    let enabled = e.mem.u8(region.addr() + 0x108) != 0;
    let region_id = e.mem.u32(region.addr());
    if enabled && region_id == e.mem.u32(0x011d_9aec) {
        return;
    }
    if enabled {
        e.mem.set_u32(0x011d_9aec, region_id);
    }
    let region_tile = hud_tile(e, 15);
    let display_tile = hud_tile(e, 16);
    let has_previous_text = if hud_float(e, region_tile, 0xfa9) != 0.0 {
        let old_text = e.call(0x00a0_11f0, &args![region_tile, 0xfc4_u32]).u32();
        e.call(0x0040_8b20, &args![region.addr() + 4, old_text])
            .i32()
            != 0
    } else {
        false
    };
    if e.mem.u8(region.addr() + 4) != 0
        && hud_float(e, display_tile, 0xfa9) == 0.0
        && e.call(0x00a0_7fc0, &args![display_tile, 0xfa9_u32]).bool()
    {
        let hold = e.call(0x00a0_1860, &args![0x0107_3474_u32]).u32();
        hud_trait(e, region_tile, hold, 0);
        hud_set_text(e, region_tile, region.addr() + 4);
        if has_previous_text {
            add_hud_fade(e, region_tile);
        }
    }
}

// Translated from 00772aa0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Replaces the HUD's location text when a newly active reference has map
/// marker data, and clears its fade control while the region text resets.
pub fn fn_00772aa0(e: &mut Engine, reference: Ptr, state: Ptr) {
    if e.mem.u32(0x011d_96c0) == 0 {
        return;
    }
    let active = e.mem.u8(state.addr() + 4) != 0;
    let id = e.mem.u32(state.addr());
    if active && id != e.mem.u32(0x011d_9af0) {
        e.mem.set_u32(0x011d_9af0, id);
        if id == 0 {
            return;
        }
        let marker = e.call(0x0056_9060, &args![id]).u32();
        if marker == 0 {
            return;
        }
        let display_tile = hud_tile(e, 16);
        if e.call(0x00a0_7fc0, &args![display_tile, 0xfa9_u32]).bool()
            && hud_float(e, display_tile, 0xfa9) == 0.0
        {
            let marker = e.call(0x0056_9060, &args![id]).u32();
            let name = e.call(0x0043_8eb0, &args![marker]).u32();
            hud_set_text(e, display_tile, name);
            add_hud_fade(e, display_tile);
        }
        let region_tile = hud_tile(e, 15);
        e.call(0x00a0_7dc0, &args![region_tile, 0xfa9_u32]);
        hud_set_float(e, region_tile, 0xfa9, 0.0);
    } else if !active && id == e.mem.u32(0x011d_9af0) {
        e.mem.set_u32(0x011d_9af0, 0);
    } else if e.mem.u8(state.addr() + 5) != 0 {
        e.mem.set_u32(0x011d_9af0, id);
    }
    let _ = reference; // ECX is not read by the PC body.
}

// Translated from 00772c30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::SetWorldspaceText` (Xbox PDB): shows the worldspace name
/// after the location tile has faded completely. SEH bookkeeping is omitted.
pub fn hud_main_menu_set_worldspace_text(e: &mut Engine, text: Ptr) {
    if e.mem.u32(0x011d_9af8) & 1 == 0 {
        let flags = e.mem.u32(0x011d_9af8);
        e.mem.set_u32(0x011d_9af8, flags | 1);
        let hold = e.call(0x00a0_1860, &args![0x0107_3474_u32]).u32();
        e.mem.set_u32(0x011d_9af4, hold);
    }
    if text.is_null() {
        return;
    }
    let display_tile = hud_tile(e, 16);
    if hud_float(e, display_tile, 0xfa9) == 0.0
        && e.call(0x00a0_7fc0, &args![display_tile, 0xfa9_u32]).bool()
    {
        let region_tile = hud_tile(e, 15);
        hud_set_text(e, region_tile, text.addr());
        let hold = e.call(0x00a0_1860, &args![0x0107_3474_u32]).u32();
        hud_trait(e, region_tile, hold, 1);
    }
}

// Translated from 00773050 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the current enemy, quest timer, experience state and associated
/// tile visibility when the HUD menu exists.
pub fn fn_00773050(e: &mut Engine) {
    let class = e.call(0x0070_6f20, &args![]).u32();
    let menu_tile = e.call(0x00a0_9030, &args![class]).u32();
    if menu_tile == 0 {
        return;
    }
    let menu = e.call(0x00a0_3c90, &args![menu_tile]).u32();
    if menu == 0 {
        return;
    }
    let menu = e.call(0x00a0_3c90, &args![menu_tile]).u32();
    e.mem.set_u32(menu + 0x21c, 0);
    e.mem.set_u32(menu + 0x248, 0);
    e.mem.set_u32(menu + 0x24c, u32::MAX);
    e.mem.set_u32(menu + 0x250, u32::MAX);
    e.mem.set_u8(menu + 0x205, 0);
    let hotkey = e.mem.u32(menu + 0xf0);
    hud_trait(e, hotkey, 0xfa3, 0);
    e.mem.set_u8(menu + 0x204, 0);
    for id in 0x2c..=0x31 {
        let tile = hud_tile(e, id);
        hud_trait(e, tile, 0xfa3, 0);
    }
    e.mem.set_u32(0x011a_010c, 1);
    let singleton = e.mem.u32(0x011d_96c0);
    e.mem.set_u8(singleton + 0x254, 1);
}

// Translated from 00773190 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::SetEnemyActor` (Xbox PDB): chooses the enemy shown by the
/// HUD, updates its name, and starts the health timer. A static `cdecl` of one word
/// (its callers 00770ded, 00772ec2 and 007038b7 push only the enemy and pop one word; it
/// does not read ECX).
pub fn hud_main_menu_set_enemy_actor(e: &mut Engine, enemy: Ptr) {
    let blocked = e.call(0x0040_8d60, &args![0x011e_0988_u32]).u32();
    if e.mem.u8(blocked) != 0
        || (!enemy.is_null() && e.vcall(enemy.addr(), 0x22c, &args![0_u32]).bool())
    {
        return;
    }
    let menu = e.mem.u32(0x011d_96c0);
    if menu == 0 {
        return;
    }
    let mode = e.call(0x0044_ddc0, &args![0x011f_2250_u32]).i32();
    let chosen = if mode == 4 {
        let alternative = e.mem.u32(0x011f_21cc);
        if alternative != 0 && e.vcall(alternative, 0x100, &args![]).bool() {
            alternative
        } else {
            0
        }
    } else {
        enemy.addr()
    };
    e.mem.set_u32(menu + 0x21c, chosen);
    if chosen != 0 {
        let name = e.call(0x0055_d520, &args![chosen]).u32();
        let current_menu = e.mem.u32(0x011d_96c0);
        let name_tile = e.mem.u32(current_menu + 0x84);
        hud_set_text(e, name_tile, name);
        let current_menu = e.mem.u32(0x011d_96c0);
        fn_00771520(e, Ptr::new(current_menu + 0x224));
        let current_menu = e.mem.u32(0x011d_96c0);
        let now = e.call(0x0082_5c00, &args![current_menu + 0x224]).u32();
        let current_menu = e.mem.u32(0x011d_96c0);
        e.mem.set_u32(current_menu + 0x244, now);
    }
}

// Translated from 007732b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the HUD's current enemy, if the HUD exists.
pub fn fn_007732b0(e: &mut Engine) -> Ptr {
    let menu = e.mem.u32(0x011d_96c0);
    Ptr::new(if menu == 0 {
        0
    } else {
        e.mem.u32(menu + 0x21c)
    })
}

// Translated from 007735b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the player state byte at +0x5f8.
pub fn fn_007735b0(e: &mut Engine, player: Ptr) -> u8 {
    e.mem.u8(player.addr() + 0x5f8)
}

fn quest_text_showing(
    e: &mut Engine,
    this: Ptr<HUDMainMenu>,
    flag_offset: u32,
    tile_id: u32,
) -> bool {
    if e.mem.u8(this.addr() + flag_offset) == 0 {
        return false;
    }
    let singleton = e.mem.u32(0x011d_96c0);
    let tile = e.mem.u32(singleton + 0x28 + tile_id * 4);
    e.with_stack(4, |e, cursor| {
        let first = e.call(0x0055_9450, &args![tile + 4]).u32();
        e.mem.set_u32(cursor.addr(), first);
        while e.mem.u32(cursor.addr()) != 0 {
            let item = e.call(0x0057_cbe0, &args![tile + 4, cursor]).u32();
            let text_tile = e.mem.u32(item);
            if e.call(0x00a0_1230, &args![text_tile, 0xfa3_u32]).bool() {
                return true;
            }
        }
        e.mem.set_u8(this.addr() + flag_offset, 0);
        false
    })
}

// Translated from 007735d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::IsQuestNameShowing` (Xbox PDB): tests quest-name child
/// tiles and clears the cached showing flag once none is visible.
pub fn hud_main_menu_is_quest_name_showing(e: &mut Engine, this: Ptr<HUDMainMenu>) -> bool {
    quest_text_showing(e, this, 0x260, 0x0e)
}

// Translated from 00773650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::AreObjectivesShowing` (Xbox PDB): tests objective child
/// tiles and clears the cached showing flag once none is visible.
pub fn hud_main_menu_are_objectives_showing(e: &mut Engine, this: Ptr<HUDMainMenu>) -> bool {
    quest_text_showing(e, this, 0x261, 0x0d)
}

// Translated from 007737a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests the requested flag in the object at +0x20.
pub fn fn_007737a0(e: &mut Engine, object: Ptr, flag: u32) -> bool {
    e.mem.u32(object.addr() + 0x20) & flag != 0
}

// Translated from 007736d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::DisplayQuestReminder` (Xbox PDB): queues incomplete,
/// eligible objectives from the quest's target list while reminders are free.
pub fn hud_main_menu_display_quest_reminder(e: &mut Engine, quest: Ptr) {
    let menu = e.mem.u32(0x011d_96c0);
    if menu == 0 || quest.is_null() {
        return;
    }
    if hud_main_menu_are_objectives_showing(e, Ptr::new(menu))
        || !e.call(0x0082_56d0, &args![menu + 0x258]).bool()
    {
        return;
    }
    let mut node = e.call(0x005d_43e0, &args![quest]).u32();
    while node != 0 && !e.call(0x0082_56d0, &args![node]).bool() {
        let element = e.call(0x0068_15c0, &args![node]).u32();
        let objective = e.mem.u32(element);
        if fn_007737a0(e, Ptr::new(objective), 1) {
            let element = e.call(0x0068_15c0, &args![node]).u32();
            let objective = e.mem.u32(element);
            if !fn_007737a0(e, Ptr::new(objective), 2) {
                let element = e.call(0x0068_15c0, &args![node]).u32();
                let objective = e.mem.u32(element);
                let data = e.call(0x005e_c330, &args![objective]).u32();
                e.call(0x0077_a5b0, &args![data, 0_u32, 1_u32]);
            }
        }
        node = e.call(0x0072_6070, &args![node]).u32();
    }
}

// Translated from 007737c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::UpdateRadiationMeterConditions` (Xbox PDB): sets the
/// radiation amount and postpones the meter's expiry when radiation rises.
pub fn hud_main_menu_update_radiation_meter_conditions(
    e: &mut Engine,
    _this: Ptr<HUDMainMenu>,
    radiation: f32,
    accumulated: f32,
) {
    let sum = (f64::from(radiation) + f64::from(accumulated)) as f32;
    let meter = hud_tile(e, 0x13);
    if radiation > 0.0 {
        let now = e.call(0x0082_5c00, &args![0x011f_6394_u32]).u32();
        let delay: f64 = e.global(0x0107_3490);
        hud_set_float(e, meter, 0x100a, (now as f64 + delay) as f32);
        hud_set_float(e, meter, 0x1009, sum);
        hud_trait(e, meter, 0x100b, 1);
    } else if accumulated > 0.0 {
        let expiry = hud_float(e, meter, 0x100a);
        let now = e.call(0x0082_5c00, &args![0x011f_6394_u32]).u32();
        if f64::from(now) >= f64::from(expiry) {
            hud_set_float(e, meter, 0x1009, sum);
        }
    }
}

// Translated from 007738c0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 007738c0 - the compiler's SEH unwind frame is omitted.
/// `HUDMainMenu::SetRadiationLevel` (Xbox PDB): starts or fades the
/// radiation sound and updates the meter for positive radiation.
/// The compiler's SEH/unwind bookkeeping is omitted.
pub fn hud_main_menu_set_radiation_level(e: &mut Engine, radiation: f32, accumulated: f32) {
    let sum = (f64::from(radiation) + f64::from(accumulated)) as f32;
    let menu = e.mem.u32(0x011d_96c0);
    if menu == 0 {
        return;
    }
    let sound = menu + 0x1ac;
    if sum <= 0.0 {
        if e.call(0x00ad_8ce0, &args![sound]).bool() {
            e.call(0x00ad_8da0, &args![sound, 500_u32]);
        }
    } else {
        let valid = e.call(0x00ad_8ce0, &args![sound]).bool();
        if !valid && accumulated > 0.0 {
            e.with_stack(12, |e, handle| {
                let instance = e.call(0x0045_3a70, &args![]).u32();
                let sample = e
                    .call(
                        0x00ad_7550,
                        &args![instance, handle, 0x0107_3498_u32, 0x11_u32],
                    )
                    .u32();
                e.call(0x0041_8900, &args![sound, sample]);
                e.call(0x0048_3710, &args![handle]);
            });
            e.call(0x00ad_8830, &args![sound, 0_u32]);
        }
        hud_main_menu_update_radiation_meter_conditions(e, Ptr::new(menu), radiation, accumulated);
    }
}

// Translated from 00774800 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::SetBrightness` (Xbox PDB): sets a tile's brightness for
/// the requested HUD role. Role 4 writes both 44 and 100 in that order.
pub fn hud_main_menu_set_brightness(e: &mut Engine, target: Ptr, role: u32) {
    match role {
        1 | 3 => hud_trait(e, target.addr(), 0xff5, 0xaf),
        2 => hud_trait(e, target.addr(), 0xff5, 0xff),
        4 => {
            hud_trait(e, target.addr(), 0xff5, 0x2c);
            hud_trait(e, target.addr(), 0xff5, 100);
        }
        5 => hud_trait(e, target.addr(), 0xff5, 100),
        _ => hud_trait(e, target.addr(), 0xff5, 0xff),
    }
}

fn x87_to_int(e: &mut Engine, value: f64) -> i32 {
    e.call(0x00ec_62c0, &args![value]).i32()
}

// Translated from 007748b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::UpdateMeter` (Xbox PDB): fits the requested fraction into
/// an integral number of image widths, then positions the meter for its
/// left-, right- or center-anchored mode.
/// The compiler's SEH/unwind bookkeeping is omitted.
pub fn hud_main_menu_update_meter(
    e: &mut Engine,
    target: Ptr,
    mut fraction: f32,
    anchor: i32,
    minimum_images: i32,
    x_offset: i32,
) {
    for (bit, name, slot) in [
        (1_u32, 0x0107_3424_u32, 0x011d_9c14_u32),
        (2, 0x0107_3418, 0x011d_9c10),
        (4, 0x0107_3430, 0x011d_9c0c),
    ] {
        let flags = e.mem.u32(0x011d_9c18);
        if flags & bit == 0 {
            e.mem.set_u32(0x011d_9c18, flags | bit);
            let trait_id = e.call(0x00a0_0940, &args![name, u32::MAX]).u32();
            e.mem.set_u32(slot, trait_id);
        }
    }
    if target.is_null() || e.mem.u32(0x011d_96c0) == 0 {
        return;
    }
    let one: f64 = e.global(0x0101_2070);
    if f64::from(fraction) > one {
        fraction = 1.0;
    } else if f64::from(fraction) < e.global::<f64>(0x0101_2060) {
        fraction = 0.0;
    }
    let width_trait = e.mem.u32(0x011d_9c14);
    let width_raw = e.call(0x00a0_11b0, &args![target, width_trait]).f64();
    let width = x87_to_int(e, width_raw);
    let image_trait = e.mem.u32(0x011d_9c10);
    let image_raw = e.call(0x00a0_11b0, &args![target, image_trait]).f64();
    let image = x87_to_int(e, image_raw);
    let zero: f64 = e.global(0x0101_2060);
    let positive = if f64::from(fraction) >= zero {
        fraction
    } else {
        0.0
    };
    let scale = if anchor == 2 {
        image.wrapping_shl(1)
    } else {
        image
    };
    let quotient = x87_to_int(e, f64::from(width) * f64::from(positive) / f64::from(scale));
    let mut displayed_width = quotient.wrapping_mul(scale);
    let minimum_width = image.wrapping_mul(minimum_images);
    if displayed_width < minimum_width {
        displayed_width = minimum_width;
    }
    let x_trait = e.mem.u32(0x011d_9c0c);
    let x_raw = e.call(0x00a0_11b0, &args![target, x_trait]).f64();
    let mut position = x87_to_int(e, x_raw);
    if anchor == 1 {
        position = position.wrapping_sub(displayed_width);
    } else if anchor == 2 {
        position = position.wrapping_sub(displayed_width / 2);
    }
    position = position.wrapping_add(x_offset);
    hud_trait(e, target.addr(), 0xfb1, displayed_width as u32);
    hud_trait(e, target.addr(), 0xfa1, position as u32);
}

// Translated from 00774b00 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00774b00 - the compiler's SEH unwind frame is omitted.
/// `HUDMainMenu::UpdateMeter_ov2` (Xbox PDB): sizes and positions two
/// overlapping meter tiles, including the center anchor's child tile and
/// alpha fades.
pub fn hud_main_menu_update_meter_ov2(
    e: &mut Engine,
    first: Ptr,
    second: Ptr,
    mut first_fraction: f32,
    mut second_fraction: f32,
    anchor: i32,
    minimum_images: i32,
) {
    for (bit, name, slot) in [
        (1_u32, 0x0107_3424_u32, 0x011d_9c28_u32),
        (2, 0x0107_3418, 0x011d_9c24),
        (4, 0x0107_3430, 0x011d_9c20),
        (8, 0x0107_34b8, 0x011d_9c1c),
    ] {
        let flags = e.mem.u32(0x011d_9c2c);
        if flags & bit == 0 {
            e.mem.set_u32(0x011d_9c2c, flags | bit);
            let trait_id = e.call(0x00a0_0940, &args![name, u32::MAX]).u32();
            e.mem.set_u32(slot, trait_id);
        }
    }
    if first.is_null() || second.is_null() || e.mem.u32(0x011d_96c0) == 0 {
        return;
    }
    for fraction in [&mut first_fraction, &mut second_fraction] {
        if f64::from(*fraction) > e.global::<f64>(0x0101_2070) {
            *fraction = 1.0;
        } else if f64::from(*fraction) < e.global::<f64>(0x0101_2060) {
            *fraction = 0.0;
        }
    }
    let total_trait = e.mem.u32(0x011d_9c28);
    let total_raw = e.call(0x00a0_11b0, &args![first, total_trait]).f64();
    let total_width = x87_to_int(e, total_raw);
    let image_trait = e.mem.u32(0x011d_9c24);
    let image_raw = e.call(0x00a0_11b0, &args![first, image_trait]).f64();
    let image_width = x87_to_int(e, image_raw);
    let scale = if anchor == 2 {
        image_width.wrapping_shl(1)
    } else {
        image_width
    };
    let first_positive = if f64::from(first_fraction) >= e.global::<f64>(0x0101_2060) {
        first_fraction
    } else {
        0.0
    };
    let _first_width_before_second_conversion = x87_to_int(
        e,
        f64::from(total_width) * f64::from(first_positive) / f64::from(scale),
    )
    .wrapping_mul(scale);
    let mut second_width = x87_to_int(
        e,
        f64::from(total_width) * f64::from(second_fraction) / f64::from(scale),
    )
    .wrapping_mul(scale);
    // The PC body recalculates the first width after both conversions, so
    // preserve this callback-visible ordering rather than reuse first_width.
    let first_positive = if f64::from(first_fraction) >= e.global::<f64>(0x0101_2060) {
        first_fraction
    } else {
        0.0
    };
    let first_width = x87_to_int(
        e,
        f64::from(total_width) * f64::from(first_positive) / f64::from(scale),
    )
    .wrapping_mul(scale);
    let original_x_trait = e.mem.u32(0x011d_9c20);
    let original_x_raw = e.call(0x00a0_11b0, &args![first, original_x_trait]).f64();
    let mut first_x = x87_to_int(e, original_x_raw);
    if anchor == 1 {
        first_x = first_x.wrapping_sub(first_width);
    } else if anchor == 2 {
        first_x = first_x.wrapping_sub(first_width / 2);
    }
    let original_x_trait = e.mem.u32(0x011d_9c20);
    let original_x_raw = e.call(0x00a0_11b0, &args![second, original_x_trait]).f64();
    let mut second_x = x87_to_int(e, original_x_raw);
    if anchor == 1 {
        let width_difference = second_width.wrapping_sub(first_width);
        second_x = first_x.wrapping_sub(width_difference);
        let original_x_trait = e.mem.u32(0x011d_9c20);
        let original_x_raw = e.call(0x00a0_11b0, &args![second, original_x_trait]).f64();
        let original_x = x87_to_int(e, original_x_raw);
        second_width = e
            .call(
                0x004a_8f20,
                &args![width_difference, original_x.wrapping_sub(second_x)],
            )
            .i32();
    } else if anchor == 0 {
        second_x = second_x.wrapping_add(first_width);
    } else if anchor == 2 {
        second_x = second_x.wrapping_sub(second_width / 2);
        second_width = first_x.wrapping_sub(second_x);
        let child_storage = e.call(0x0073_ac90, &args![second.addr() + 4]).u32();
        let child = e.mem.u32(child_storage);
        hud_trait(e, child, 0xfb1, second_width as u32);
        hud_trait(e, child, 0xfa1, first_x.wrapping_add(first_width) as u32);
        if !e.call(0x00a0_7ed0, &args![child, 0xfa9_u32]).bool() {
            let duration: f32 = quiet_f32(e.global(0x0102_3cd8));
            e.call(
                0x00a0_7c60,
                &args![child, 0xfa9_u32, 0.0_f32, duration, 1.0_f32, 1_u32],
            );
        }
    }
    if first_width == 0 && second_width < image_width.wrapping_mul(minimum_images) {
        second_width = image_width.wrapping_mul(minimum_images);
    }
    hud_trait(e, first.addr(), 0xfb1, first_width as u32);
    hud_trait(e, first.addr(), 0xfa1, first_x as u32);
    hud_trait(e, second.addr(), 0xfb1, second_width as u32);
    hud_trait(e, second.addr(), 0xfa1, second_x as u32);
    if !e.call(0x00a0_7ed0, &args![second, 0xfa9_u32]).bool() {
        let duration: f32 = quiet_f32(e.global(0x0102_3cd8));
        e.call(
            0x00a0_7c60,
            &args![second, 0xfa9_u32, 0.0_f32, duration, 1.0_f32, 1_u32],
        );
    }
}

// Translated from 00774fd0 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00774fd0 - the compiler's SEH unwind frame is omitted.
/// `HUDMainMenu::ShowSubtitle` (Xbox PDB): queues a subtitle when the text
/// and actor/setting gate allow it, deriving its duration from the sound
/// handle or the configured per-character fallback.
#[allow(clippy::too_many_arguments)]
pub fn hud_main_menu_show_subtitle(
    e: &mut Engine,
    this: Ptr<HUDMainMenu>,
    text: Ptr,
    sound_0: u32,
    sound_1: u32,
    sound_2: u32,
    point_0: u32,
    point_1: u32,
    point_2: u32,
    speaker: Ptr,
    force_priority: u8,
) -> bool {
    e.with_stack(12, |e, sound| {
        e.mem.set_u32(sound.addr(), sound_0);
        e.mem.set_u32(sound.addr() + 4, sound_1);
        e.mem.set_u32(sound.addr() + 8, sound_2);
        let mut shown = false;
        let actor = if speaker.is_null() || !e.vcall(speaker.addr(), 0x100, &args![]).bool() {
            0
        } else {
            e.call(
                0x00ec_43fb,
                &args![speaker, 0_u32, 0x0118_41cc_u32, 0x0118_46d4_u32, 0_u32],
            )
            .u32()
        };
        if !text.is_null() {
            let subtitles_setting = e.call(0x0040_8d60, &args![0x011d_8928_u32]).u32();
            let allowed = e.mem.u8(subtitles_setting) != 0
                || (actor != 0 && e.call(0x008c_1bc0, &args![actor]).bool());
            if allowed {
                let allocated = e.call(0x0040_1000, &args![0x11c_u32]).u32();
                let subtitle = if allocated == 0 {
                    0
                } else {
                    fn_00775210(e, Ptr::new(allocated)).addr()
                };
                e.call(0x0040_6d30, &args![subtitle, 0x104_u32, text]);
                for (offset, value) in [(0x10c, point_0), (0x110, point_1), (0x114, point_2)] {
                    e.mem.set_u32(subtitle + offset, value);
                }
                let duration_ms = e.call(0x00ad_8b30, &args![sound]).i32();
                let duration = (f64::from(duration_ms) / e.global::<f64>(0x0101_7b70)) as f32;
                e.mem.set_f32(subtitle + 0x108, duration);
                if f64::from(duration) == e.global::<f64>(0x0101_2060) {
                    let length = e.call(0x00ec_6130, &args![text]).u32();
                    let rate_setting = e.call(0x0040_3e20, &args![0x011d_2178_u32]).u32();
                    let fallback = (f64::from(e.mem.f32(rate_setting)) * f64::from(length)) as f32;
                    e.mem.set_f32(subtitle + 0x108, fallback);
                }
                let priority = if force_priority != 0 {
                    i32::MAX
                } else if speaker.addr() == e.mem.u32(0x011d_ea3c) {
                    0
                } else if speaker.is_null() {
                    1
                } else {
                    2
                };
                e.mem.set_i32(subtitle + 0x118, priority);
                let now = e.call(0x0082_5c00, &args![this.addr() + 0x224]).u32();
                e.mem.set_u32(subtitle + 0x104, now);
                e.with_stack(4, |e, pointer| {
                    e.mem.set_u32(pointer.addr(), subtitle);
                    e.call(0x007c_b2e0, &args![this.addr() + 0x198, pointer]);
                });
                shown = true;
            }
        }
        e.call(0x0048_3710, &args![sound]);
        shown
    })
}

// Translated from 00775210 (decompiled, FalloutNV.exe 1.4.0.525)
/// Initializes the three-word point member at +0x10c in the newly
/// allocated subtitle record, then returns that record.
pub fn fn_00775210(e: &mut Engine, subtitle: Ptr) -> Ptr {
    e.call(0x0068_15c0, &args![subtitle.addr() + 0x10c]);
    subtitle
}

// Translated from 00775230 (decompiled, FalloutNV.exe 1.4.0.525)
/// Chooses the subtitle record by squared distance from the player,
/// priority and the forced-priority sentinel. The returned pointer remains
/// in the menu's subtitle array.
pub fn fn_00775230(e: &mut Engine, this: Ptr<HUDMainMenu>) -> Ptr {
    let mut selected = 0;
    let mut best_distance: f32 = quiet_f32(e.global(0x0101_6970));
    let player = e.mem.u32(0x011d_ea3c);
    let player_position = e.vcall(player, 0x1f4, &args![]).u32();
    e.with_stack(24, |e, scratch| {
        let difference = scratch.addr();
        let copied_position = difference + 12;
        for offset in [0_u32, 4, 8] {
            e.mem.set_u32(
                copied_position + offset,
                e.mem.u32(player_position + offset),
            );
        }
        let mut index = 0_u32;
        loop {
            let count = e.call(0x0044_ddc0, &args![this.addr() + 0x198]).u32();
            if index >= count {
                break;
            }
            let element = e
                .call(0x0087_7a30, &args![this.addr() + 0x198, index])
                .u32();
            let subtitle = e.mem.u32(element);
            let vector = e
                .call(
                    0x0043_9ef0,
                    &args![subtitle + 0x10c, difference, copied_position],
                )
                .u32();
            let distance = e.call(0x004a_7290, &args![vector]).f64() as f32;
            let in_range = if selected == 0 {
                let threshold: f32 = quiet_f32(e.global(0x0101_3d84));
                let threshold = e.call(0x0068_5d20, &args![threshold, 2_u32]).f64();
                f64::from(distance) <= threshold
            } else {
                true
            };
            let mut choose = selected == 0 && in_range;
            if selected != 0 && distance < best_distance {
                let element = e
                    .call(0x0087_7a30, &args![this.addr() + 0x198, index])
                    .u32();
                let subtitle = e.mem.u32(element);
                choose = e.mem.i32(subtitle + 0x118) >= e.mem.i32(selected + 0x118);
            }
            if !choose {
                let element = e
                    .call(0x0087_7a30, &args![this.addr() + 0x198, index])
                    .u32();
                let subtitle = e.mem.u32(element);
                choose = e.mem.i32(subtitle + 0x118) == i32::MAX;
            }
            if choose {
                let element = e
                    .call(0x0087_7a30, &args![this.addr() + 0x198, index])
                    .u32();
                selected = e.mem.u32(element);
                best_distance = distance;
            }
            index = index.wrapping_add(1);
        }
    });
    Ptr::new(selected)
}

// Translated from 00775380 (decompiled, FalloutNV.exe 1.4.0.525)
// Translation incomplete: 00775380 - the compiler's SEH unwind frame is omitted.
/// Queues a HUD corner message, choosing its default icon from the PC jump
/// table when no path is supplied, and replacing or appending the current
/// message according to the list state.
#[allow(clippy::too_many_arguments)]
pub fn fn_00775380(
    e: &mut Engine,
    this: Ptr<HUDMainMenu>,
    text: Ptr,
    icon_kind: u32,
    icon_path: Ptr,
    sound_name: Ptr,
    duration: f32,
    urgent: u8,
) -> bool {
    if text.is_null() {
        return false;
    }
    let record = e.call(0x0040_1000, &args![0x310_u32]).u32();
    e.call(0x0040_6d30, &args![record, 0x104_u32, text]);
    e.mem.set_f32(record + 0x30c, quiet_f32(duration));
    if !icon_path.is_null() && e.mem.u8(icon_path.addr()) != 0 {
        e.call(0x0040_6d30, &args![record + 0x104, 0x104_u32, icon_path]);
    } else {
        // The four cases are ordered by the jump table at 00775658.
        let default_icon = match icon_kind {
            0 => Some(0x0102_08e0_u32),
            1 => Some(0x0103_a830),
            2 => Some(0x0102_08a0),
            3 => Some(0x0104_bfe8),
            _ => None,
        };
        if let Some(path) = default_icon {
            e.call(0x0040_6d30, &args![record + 0x104, 0x104_u32, path]);
        }
    }
    if sound_name.is_null() {
        e.mem.set_u8(record + 0x208, 0);
    } else {
        e.call(0x0040_6d30, &args![record + 0x208, 0x104_u32, sound_name]);
    }
    e.with_stack(4, |e, record_pointer| {
        e.mem.set_u32(record_pointer.addr(), record);
        let list = this.addr() + 0x18c;
        if e.call(0x0082_56d0, &args![list]).bool() {
            e.call(0x0090_5820, &args![list, record_pointer]);
            let first_node = e.call(0x0068_15c0, &args![list]).u32();
            let first_record = e.mem.u32(first_node);
            let menu = e.mem.u32(0x011d_96c0);
            e.call(
                0x00a0_1350,
                &args![e.mem.u32(menu + 0x90), 0xfc4_u32, first_record, 1_u32],
            );
            let first_node = e.call(0x0068_15c0, &args![list]).u32();
            let first_record = e.mem.u32(first_node);
            let menu = e.mem.u32(0x011d_96c0);
            e.call(
                0x00a0_1350,
                &args![
                    e.mem.u32(menu + 0x8c),
                    0xfcc_u32,
                    first_record + 0x104,
                    1_u32
                ],
            );
            if !sound_name.is_null() {
                e.with_stack(12, |e, sound| {
                    let instance = e.call(0x0045_3a70, &args![]).u32();
                    let handle = e
                        .call(0x00ad_7550, &args![instance, sound, sound_name, 0x121_u32])
                        .u32();
                    e.call(0x00ad_8830, &args![handle, 0_u32]);
                    e.call(0x0048_3710, &args![sound]);
                });
            }
        } else {
            let mut tail = list;
            loop {
                let next = e.call(0x0072_6070, &args![tail]).u32();
                if next == 0 {
                    break;
                }
                tail = e.call(0x0072_6070, &args![tail]).u32();
            }
            let node = e.call(0x0068_15c0, &args![tail]).u32();
            let old_record = e.mem.u32(node);
            let duplicate = e.call(0x0040_8b20, &args![old_record, record]).i32() == 0;
            if duplicate {
                e.call(0x0040_1030, &args![record]);
            } else if urgent == 0 {
                e.call(0x0090_5820, &args![list, record_pointer]);
            } else {
                let first_node = e.call(0x0068_15c0, &args![list]).u32();
                let first_record = e.mem.u32(first_node);
                e.mem.set_f32(first_record + 0x30c, 0.0);
                e.call(0x005a_e3d0, &args![list, record_pointer]);
            }
        }
    });
    true
}

// Translated from 00775670 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::HideSubtitle` (Xbox PDB): clears the visible subtitle tile
/// and the menu's current subtitle pointer, updating its text when the
/// dialogue menu is not visible.
pub fn hud_main_menu_hide_subtitle(e: &mut Engine, this: Ptr<HUDMainMenu>) {
    if !e.call(0x0070_2680, &args![0x3f5_u32, 0_u32]).bool() {
        let hud = e.mem.u32(0x011d_96c0);
        let subtitle_tile = e.mem.u32(hud + 0x98);
        e.call(
            0x00a0_1350,
            &args![subtitle_tile, 0xfc4_u32, 0x0102_0770_u32, 1_u32],
        );
    }
    let hud = e.mem.u32(0x011d_96c0);
    let subtitle_tile = e.mem.u32(hud + 0x98);
    hud_trait(e, subtitle_tile, 0xfa3, 0);
    e.mem.set_u32(this.addr() + 0x1a8, 0);
}

// Translated from 007756e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::HideMessage` (Xbox PDB): removes the currently displayed
/// message from the linked list, then shows the next one or fades the three
/// message tiles when the list becomes empty.
pub fn hud_main_menu_hide_message(e: &mut Engine, this: Ptr<HUDMainMenu>) {
    let list = this.addr() + 0x18c;
    if e.call(0x0082_56d0, &args![list]).bool() {
        return;
    }
    let mut node = list;
    while node != 0 {
        let node_value = e.call(0x0068_15c0, &args![node]).u32();
        let record = e.mem.u32(node_value);
        if record != 0 {
            let menu = e.mem.u32(0x011d_96c0);
            let tile = e.mem.u32(menu + 0x90);
            let displayed = e.call(0x00a0_11f0, &args![tile, 0xfc4_u32]).u32();
            if e.call(0x0040_8b20, &args![record, displayed]).i32() == 0 {
                e.with_stack(4, |e, record_pointer| {
                    e.mem.set_u32(record_pointer.addr(), record);
                    e.call(0x0090_5330, &args![list, record_pointer]);
                });
                e.call(0x0040_1030, &args![record]);
                if !e.call(0x0082_56d0, &args![list]).bool() {
                    let first_node = e.call(0x0068_15c0, &args![list]).u32();
                    let first_record = e.mem.u32(first_node);
                    if e.mem.u8(first_record + 0x208) != 0 {
                        let first_node = e.call(0x0068_15c0, &args![list]).u32();
                        let first_record = e.mem.u32(first_node);
                        e.with_stack(12, |e, sound| {
                            let player = e.mem.u32(0x011d_ea3c);
                            e.call(
                                0x0093_3270,
                                &args![
                                    player,
                                    sound,
                                    first_record + 0x208,
                                    0_u32,
                                    0x121_u32,
                                    0_u32
                                ],
                            );
                            e.call(0x0048_3710, &args![sound]);
                        });
                    }
                    let first_node = e.call(0x0068_15c0, &args![list]).u32();
                    let first_record = e.mem.u32(first_node);
                    let menu = e.mem.u32(0x011d_96c0);
                    let tile = e.mem.u32(menu + 0x90);
                    e.call(0x00a0_1350, &args![tile, 0xfc4_u32, first_record, 1_u32]);
                    let first_node = e.call(0x0068_15c0, &args![list]).u32();
                    let first_record = e.mem.u32(first_node);
                    let menu = e.mem.u32(0x011d_96c0);
                    let tile = e.mem.u32(menu + 0x8c);
                    e.call(
                        0x00a0_1350,
                        &args![tile, 0xfcc_u32, first_record + 0x104, 1_u32],
                    );
                    let now = e.call(0x0082_5c00, &args![this.addr() + 0x224]).u32();
                    e.mem.set_u32(this.addr() + 0x194, now);
                } else {
                    for offset in [0x90_u32, 0x8c, 0x94] {
                        let start = quiet_f32(e.mem.f32(0x011d_979c));
                        let duration: f32 = quiet_f32(e.global(0x0102_1f44));
                        let menu = e.mem.u32(0x011d_96c0);
                        let tile = e.mem.u32(menu + offset);
                        e.call(
                            0x00a0_7c60,
                            &args![tile, 0xfa9_u32, start, 0.0_f32, duration, 0_u32],
                        );
                    }
                    e.mem.set_u32(this.addr() + 0x194, 0);
                }
                return;
            }
        }
        node = e.call(0x0072_6070, &args![node]).u32();
    }
}

// Translated from 00775920 (decompiled, FalloutNV.exe 1.4.0.525)
/// Frees every queued HUD message, clears the message tiles and hides the
/// subtitle in the order used by the PC menu.
pub fn fn_00775920(e: &mut Engine, this: Ptr<HUDMainMenu>) {
    let list = this.addr() + 0x18c;
    let mut node = list;
    while node != 0 {
        if e.call(0x0082_56d0, &args![node]).bool() {
            break;
        }
        let node_value = e.call(0x0068_15c0, &args![node]).u32();
        let record = e.mem.u32(node_value);
        e.call(0x0040_1030, &args![record]);
        node = e.call(0x0072_6070, &args![node]).u32();
    }
    e.call(0x0047_0470, &args![list]);
    let hud = e.mem.u32(0x011d_96c0);
    let tile = e.mem.u32(hud + 0x94);
    hud_set_float(e, tile, 0xfa9, 0.0);
    let hud = e.mem.u32(0x011d_96c0);
    let tile = e.mem.u32(hud + 0x90);
    hud_set_text(e, tile, 0x0102_0770);
    let hud = e.mem.u32(0x011d_96c0);
    let tile = e.mem.u32(hud + 0x90);
    hud_set_float(e, tile, 0xfa9, 0.0);
    let hud = e.mem.u32(0x011d_96c0);
    let tile = e.mem.u32(hud + 0x8c);
    hud_set_float(e, tile, 0xfa9, 0.0);
    hud_main_menu_hide_subtitle(e, this);
}

// Translated from 00773e90 (decompiled, FalloutNV.exe 1.4.0.525)
/// Displays a status icon and its alpha according to four configured
/// thresholds, then advances its shared blinking alpha when requested.
pub fn fn_00773e90(
    e: &mut Engine,
    _this: Ptr<HUDMainMenu>,
    target: Ptr,
    value: f32,
    advance_blink: bool,
) {
    let threshold = |e: &mut Engine, setting| {
        let address = e.call(0x0040_3e20, &args![setting]).u32();
        e.mem.f32(address)
    };
    let high = threshold(e, 0x011d_07fc_u32);
    if value >= high {
        hud_trait(e, target.addr(), 0xff4, 2);
        let alpha = e.mem.f32(0x011d_9b04);
        hud_set_float(e, target.addr(), 0xfa9, alpha);
    } else if value >= threshold(e, 0x011d_132c_u32) {
        hud_trait(e, target.addr(), 0xff4, 2);
        hud_trait(e, target.addr(), 0xfa9, 0xff);
    } else if value >= threshold(e, 0x011d_0c84_u32) {
        hud_trait(e, target.addr(), 0xff4, 1);
        hud_trait(e, target.addr(), 0xfa9, 0xff);
    } else if value >= threshold(e, 0x011d_05a4_u32) {
        hud_trait(e, target.addr(), 0xff4, 1);
        hud_trait(e, target.addr(), 0xfa9, 0x7c);
    } else {
        hud_trait(e, target.addr(), 0xff4, 1);
        hud_trait(e, target.addr(), 0xfa9, 0);
    }
    if !advance_blink {
        return;
    }
    let now = e.call(0x0082_5c00, &args![0x011f_6394_u32]).u32();
    let elapsed = now.wrapping_sub(e.mem.u32(0x011d_9b00));
    e.mem.set_u32(0x011d_9b00, now);
    let rate: f32 = e.global(0x0101_6248);
    let alpha = e.mem.f32(0x011d_9b04);
    let change = f64::from(elapsed) * f64::from(rate);
    let next = if e.mem.u8(0x011d_9afc) != 0 {
        (f64::from(alpha) - change) as f32
    } else {
        (f64::from(alpha) + change) as f32
    };
    e.mem.set_f32(0x011d_9b04, next);
    if next >= e.global::<f64>(0x0101_e568) as f32 {
        e.mem.set_u8(0x011d_9afc, 1);
    }
    if next <= e.global::<f64>(0x0102_e430) as f32 {
        e.mem.set_u8(0x011d_9afc, 0);
    }
    let minimum: f32 = e.global(0x0101_e704);
    let maximum: f32 = e.global(0x0102_3cd8);
    e.call(0x0053_30e0, &args![0x011d_9b04_u32, minimum, maximum]);
}

fn radiation_bar_position(e: &mut Engine, amount: f32) -> f32 {
    let limit: f32 = e.global(0x0101_3974);
    let bounded = e.call(0x0040_ebd0, &args![amount, limit]).f32();
    let bounded = e.call(0x0040_4010, &args![0.0_f32, bounded]).f32();
    let low: f32 = e.global(0x0101_e580);
    let high: f32 = e.global(0x0107_34b4);
    e.call(0x004b_3ab0, &args![low, high, 0.0_f32, limit, bounded])
        .f32()
}

fn radiation_fade(e: &mut Engine, target: u32, alpha: f32) {
    // The original reads the start alpha from the first radiation tile for
    // every tile in the group, including the other three.
    let start_tile = hud_tile(e, 0x11);
    let start = hud_float(e, start_tile, 0xfa9);
    let duration: f32 = e.global(0x0101_622c);
    e.call(
        0x00a0_7c60,
        &args![target, 0xfa9_u32, start, alpha, duration, 0_u32],
    );
}

// Translated from 00774090 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::UpdateRadiationMeter` (Xbox PDB): moves and fades the
/// radiation gauge during and after a radiation event.
pub fn hud_main_menu_update_radiation_meter(e: &mut Engine, _this: Ptr<HUDMainMenu>) {
    let meter = hud_tile(e, 0x13);
    let expiry = hud_float(e, meter, 0x100a);
    let now = e.call(0x0082_5c00, &args![0x011f_6394_u32]).u32();
    let pending = f64::from(now) < f64::from(expiry);
    let status = e.call(0x0050_98e0, &args![0x011e_0e80_u32]).bool();
    if !status && !pending && e.mem.u32(0x011a_0120) == 4 {
        return;
    }
    let mut transition = false;
    let zero: f64 = e.global(0x0101_2060);
    let mut all_hidden = true;
    for id in 0x11..=0x14 {
        let item = hud_tile(e, id);
        if !e.call(0x00a0_7fc0, &args![item, 0xfa9_u32]).bool()
            || f64::from(hud_float(e, item, 0xfa9)) != zero
        {
            all_hidden = false;
            break;
        }
    }
    if all_hidden {
        e.mem.set_u32(0x011a_0120, 4);
    }
    let state = e.mem.u32(0x011a_0120);
    if (state == 4 || state == 2) && (status || pending) {
        e.mem.set_u32(0x011a_0120, 8);
        transition = true;
    }
    let meter = hud_tile(e, 0x13);
    let marked = e.call(0x00a0_1230, &args![meter, 0x100b_u32]).bool();
    let pending_gauge = marked || pending;
    let mut difference = 0.0_f32;
    let mut actual_position = 0.0_f32;
    let mut difference_position = 0.0_f32;
    if status || pending || transition {
        let player = e.mem.u32(0x011d_ea3c);
        let raw = e.vcall(player + 0xa4, 8, &args![0x36_u32]).i32() as f32;
        let mut display_text = false;
        if status || transition || pending_gauge {
            display_text = true;
            let meter = hud_tile(e, 0x13);
            difference = hud_float(e, meter, 0x1009);
            difference_position =
                radiation_bar_position(e, (f64::from(raw) - f64::from(difference)) as f32);
        }
        if (pending && transition) || pending_gauge || status {
            display_text = true;
            actual_position = radiation_bar_position(e, raw);
        }
        if display_text {
            let amount = x87_to_int(e, f64::from(difference));
            let text_number = e.call(0x0064_7b70, &args![1_u32, amount]).u32();
            e.call(
                0x0040_6d00,
                &args![0x011d_9b08_u32, 0x104_u32, 0x0107_34b0_u32, text_number],
            );
            let meter = hud_tile(e, 0x13);
            hud_set_text(e, meter, 0x011d_9b08);
        }
    }
    let upper: f32 = e.global(0x0107_34b4);
    if transition {
        e.mem.set_u32(0x011a_0120, 8);
        let selected = if status {
            actual_position
        } else {
            difference_position
        };
        let limited = e.call(0x0040_ebd0, &args![selected, upper]).f32();
        let target = hud_tile(e, 0x12);
        hud_set_float(e, target, 0xfa1, limited);
        let indicator = hud_tile(e, 0x3d);
        hud_trait(e, indicator, 0xfa3, 1);
        let alpha: f32 = e.global(0x011d_979c);
        for id in 0x11..=0x14 {
            let target = hud_tile(e, id);
            radiation_fade(e, target, alpha);
        }
    } else if status {
        let limited = e.call(0x0040_ebd0, &args![actual_position, upper]).f32();
        let target = hud_tile(e, 0x12);
        hud_set_float(e, target, 0xfa1, limited);
    }
    if pending_gauge {
        let meter = hud_tile(e, 0x13);
        hud_trait(e, meter, 0x100b, 0);
        let limited = e.call(0x0040_ebd0, &args![actual_position, upper]).f32();
        let gauge = hud_tile(e, 0x12);
        hud_set_float(e, gauge, 0x1004, limited);
        let fifty: f32 = e.global(0x0101_b268);
        let limited_amount = e.call(0x0040_ebd0, &args![fifty, difference]).f32();
        let start_time: f32 = e.global(0x0101_6088);
        let end_time: f32 = e.global(0x0101_7718);
        let duration = e
            .call(
                0x004b_3ab0,
                &args![start_time, end_time, 0.0_f32, fifty, limited_amount],
            )
            .f32();
        let gauge = hud_tile(e, 0x12);
        let start = hud_float(e, gauge, 0xfa1);
        let position = hud_float(e, gauge, 0x1004);
        let floor: f32 = e.global(0x0101_e580);
        let end = e.call(0x0040_4010, &args![position, floor]).f32();
        if !status {
            let gauge = hud_tile(e, 0x12);
            e.call(
                0x00a0_7c60,
                &args![gauge, 0xfa1_u32, start, end, duration, 0_u32],
            );
        }
    }
    if !status && !pending {
        e.mem.set_u32(0x011a_0120, 2);
        for id in 0x11..=0x14 {
            let target = hud_tile(e, id);
            radiation_fade(e, target, 0.0);
        }
    }
}

fn sneak_alpha(e: &mut Engine) -> f32 {
    let setting = e.call(0x0040_3e20, &args![0x011d_9774_u32]).u32();
    (f64::from(e.mem.f32(setting)) * e.global::<f64>(0x0101_e568)) as f32
}

fn sneak_set_label(e: &mut Engine, setting: u32) {
    let text = e.call(0x0040_3df0, &args![setting]).u32();
    let tile = hud_tile(e, 0x18);
    hud_set_text(e, tile, text);
}

// Translated from 007732d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::UpdateSneak` (Xbox PDB): selects the sneak status text,
/// image and alpha fade from combat, discovery and the player state.
pub fn hud_main_menu_update_sneak(e: &mut Engine, _this: Ptr<HUDMainMenu>) {
    e.with_stack(1, |e, combat_detail| {
        e.mem.set_u8(combat_detail.addr(), 0);
        let player = e.mem.u32(0x011d_ea3c);
        let combat = e.call(0x0095_3c50, &args![player, combat_detail]).bool();
        let detail = e.mem.u8(combat_detail.addr()) != 0;
        let mut alert_fade = false;
        if combat || detail {
            let tile = hud_tile(e, 0x18);
            hud_trait(e, tile, 0xff4, 2);
            alert_fade = !detail;
            sneak_set_label(e, if detail { 0x011d_4804 } else { 0x011d_4eb8 });
        } else {
            let player = e.mem.u32(0x011d_ea3c);
            if fn_007735b0(e, Ptr::new(player)) != 0 {
                let tile = hud_tile(e, 0x18);
                hud_trait(e, tile, 0xff4, 2);
                sneak_set_label(e, 0x011d_4804);
            } else {
                let player = e.mem.u32(0x011d_ea3c);
                let detected = e
                    .call(0x0097_3710, &args![0x011e_0e80_u32, player, 0_u32])
                    .i32();
                let tile = hud_tile(e, 0x18);
                hud_trait(e, tile, 0xff4, 1);
                if detected <= 0 {
                    sneak_set_label(e, 0x011d_27b4);
                } else {
                    let player = e.mem.u32(0x011d_ea3c);
                    if fn_007735b0(e, Ptr::new(player)) == 0 {
                        sneak_set_label(e, 0x011d_2490);
                    }
                }
            }
        }
        let tile = hud_tile(e, 0x18);
        if alert_fade && !e.call(0x00a0_1230, &args![tile, 0x1004_u32]).bool() {
            let alpha = sneak_alpha(e);
            let duration: f32 = e.global(0x0101_6248);
            let tile = hud_tile(e, 0x18);
            e.call(
                0x00a0_7c60,
                &args![tile, 0xfa9_u32, 0.0_f32, alpha, duration, 2_u32],
            );
        } else {
            let fade_type = if alert_fade {
                -1
            } else {
                let tile = hud_tile(e, 0x18);
                e.call(0x00a0_8070, &args![tile, 0xfa9_u32]).i32()
            };
            let needs_fade = if !alert_fade && fade_type == 2 {
                true
            } else {
                let tile = hud_tile(e, 0x18);
                if !e.call(0x00a0_7fc0, &args![tile, 0xfa9_u32]).bool() {
                    false
                } else {
                    let tile = hud_tile(e, 0x18);
                    let current = hud_float(e, tile, 0xfa9);
                    let alpha = sneak_alpha(e);
                    current != alpha
                }
            };
            if needs_fade {
                let alpha = sneak_alpha(e);
                let duration: f32 = e.global(0x0101_6248);
                let tile = hud_tile(e, 0x18);
                let current = hud_float(e, tile, 0xfa9);
                let tile = hud_tile(e, 0x18);
                e.call(
                    0x00a0_7c60,
                    &args![tile, 0xfa9_u32, current, alpha, duration, 0_u32],
                );
            }
        }
        let tile = hud_tile(e, 0x18);
        hud_trait(e, tile, 0x1004, u32::from(alert_fade));
    });
}

// Translated from 00772d10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::SetTargetType` (Xbox PDB): decides whether the aimed actor
/// can become the health target, then updates the enemy tiles and cached
/// perceived/essential flags.
pub fn hud_main_menu_set_target_type(e: &mut Engine, target: Ptr, enabled: bool) {
    let mut perceived = false;
    let mut actor_blocked = true;
    let blocked = if target.is_null() || e.mem.u32(0x011d_96c0) == 0 {
        true
    } else {
        let blocked_ptr = e.call(0x0040_8d60, &args![0x011d_8a0c_u32]).u32();
        if e.mem.u8(blocked_ptr) != 0 || !enabled {
            true
        } else {
            let iron_ptr = e.call(0x0040_8d60, &args![0x011d_898c_u32]).u32();
            if e.mem.u8(iron_ptr) != 0 {
                let player = e.mem.u32(0x011d_ea3c);
                e.call(0x008b_bc10, &args![player]).bool()
            } else {
                false
            }
        }
    };
    if blocked {
        let icon = hud_tile(e, 0x3e);
        let one: f64 = e.global(0x0101_2070);
        if e.call(0x00a0_11b0, &args![icon, 0xff4_u32]).f64() == one {
            for id in [0x15_u32, 0x16, 0x17] {
                let item = hud_tile(e, id);
                hud_trait(e, item, 0xfa3, 0);
            }
        }
    } else {
        e.call(0x0057_9280, &args![target]);
        let actor = if e.vcall(target.addr(), 0x100, &args![]).bool() {
            target.addr()
        } else {
            0
        };
        actor_blocked = actor != 0 && e.vcall(actor, 0x22c, &args![0_u32]).bool();
        if actor != 0 {
            let player = e.mem.u32(0x011d_ea3c);
            let observation = e.call(0x0096_7090, &args![player, actor]).u32();
            if observation != 0 {
                let player = e.mem.u32(0x011d_ea3c);
                let observation = e.call(0x0096_7090, &args![player, actor]).u32();
                if e.mem.u8(observation + 4) != 0 {
                    perceived = true;
                }
            }
            if !perceived {
                let player = e.mem.u32(0x011d_ea3c);
                perceived = e.call(0x008b_c700, &args![actor, player]).bool();
            }
        }
        let mut show = false;
        if actor != 0 && !actor_blocked && !e.vcall(actor, 0x22c, &args![0_u32]).bool() {
            let current = e.vcall(actor + 0xa4, 8, &args![0x10_u32]).i32();
            let maximum = e.vcall(actor + 0xa4, 0, &args![0x10_u32]).i32();
            if current < maximum {
                let meter = hud_tile(e, 0x20);
                show = !e.call(0x00a0_1230, &args![meter, 0xfa3_u32]).bool();
            }
        }
        if show {
            hud_main_menu_set_enemy_actor(e, Ptr::new(actor));
            for id in [0x15_u32, 0x16, 0x17, 0x17] {
                let item = hud_tile(e, id);
                hud_trait(e, item, 0xfa3, 1);
            }
            let indicator = hud_tile(e, 0x3e);
            hud_trait(e, indicator, 0xff4, u32::from(perceived) + 1);
            for id in [0x15_u32, 0x16, 0x17, 0x17] {
                let item = hud_tile(e, id);
                e.call(0x00a0_7530, &args![item, 8_u32]);
            }
            let menu = e.mem.u32(0x011d_96c0);
            hud_main_menu_update_enemy_health(e, Ptr::new(menu), Ptr::new(actor));
        }
    }
    e.mem
        .set_u8(0x011d_96ba, u8::from(!actor_blocked && perceived));
    e.mem.set_u8(0x011d_96b9, u8::from(enabled));
}

fn inventory_count_for(e: &mut Engine, player: u32, item: u32) -> i32 {
    let changes = e.call(0x004b_f220, &args![player]).u32();
    e.call(0x004c_8f30, &args![changes, item]).i32()
}

// Translated from 007721c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Refreshes the equipped weapon's ammunition and condition indicators.
/// The PC body reads the selected item from `MiddleHighProcess`, including
/// a separate branch for form types 10 through 13.
pub fn fn_007721c0(e: &mut Engine, this: Ptr<HUDMainMenu>) {
    let player = e.mem.u32(0x011d_ea3c);
    if player == 0 {
        return;
    }
    let process = e.call(0x008d_8520, &args![player]).u32();
    if process == 0 {
        return;
    }
    let alpha_setting = e.call(0x0040_3e20, &args![0x011d_9774_u32]).u32();
    let alpha = (f64::from(e.mem.f32(alpha_setting)) * e.global::<f64>(0x0101_e568)) as f32;
    let player = e.mem.u32(0x011d_ea3c);
    let process = e.call(0x008d_8520, &args![player]).u32();
    let item_change = e.vcall(process, 0x148, &args![]).u32();
    if e.mem.u32(0x011d_9ae8) & 1 == 0 {
        let flags = e.mem.u32(0x011d_9ae8);
        e.mem.set_u32(0x011d_9ae8, flags | 1);
        e.mem.set_f32(0x011d_9ae4, alpha);
    }
    e.call(
        0x0040_6d00,
        &args![0x011d_99e0_u32, 0x104_u32, 0x0101_9f08_u32, 0x0101_1584_u32],
    );
    let ammo_label = e.mem.u32(this.addr() + 0x15c);
    hud_set_text(e, ammo_label, 0x011d_99e0);
    if item_change == 0 {
        let sentinel: f32 = e.global(0x0102_efc4);
        for address in [0x011a_011c_u32, 0x011a_0118, 0x011a_0110] {
            e.mem.set_f32(address, sentinel);
        }
        e.mem.set_u32(0x011d_98d4, 0);
        for address in [
            hud_tile(e, 0x0c),
            e.mem.u32(this.addr() + 0x50),
            e.mem.u32(this.addr() + 0x54),
            hud_tile(e, 0x09),
            e.mem.u32(this.addr() + 0x180),
        ] {
            hud_trait(e, address, 0xfa3, 0);
        }
        return;
    }
    let weapon = e.call(0x0044_ddc0, &args![item_change]).u32();
    let mut loaded = 0.0_f32;
    let mut reserve = 0.0_f32;
    let form_type = e.call(0x0044_6390, &args![weapon]).i32();
    if (10..=13).contains(&form_type) {
        loaded = e.call(0x0052_4b60, &args![weapon]).u8() as f32;
        let player = e.mem.u32(0x011d_ea3c);
        let total = inventory_count_for(e, player, weapon);
        reserve = (f64::from(total) - f64::from(loaded)) as f32;
    } else {
        let player = e.mem.u32(0x011d_ea3c);
        let process = e.call(0x008d_8520, &args![player]).u32();
        let acquired = e.vcall(process, 0x14c, &args![]).u32();
        let mut converted = 0;
        if acquired != 0 {
            let acquired_form = e.call(0x0044_ddc0, &args![acquired]).u32();
            if acquired_form != 0 {
                let acquired_form = e.call(0x0044_ddc0, &args![acquired]).u32();
                converted = e
                    .call(
                        0x00ec_43fb,
                        &args![
                            acquired_form,
                            0_u32,
                            0x0118_3108_u32,
                            0x0118_40c4_u32,
                            0_u32
                        ],
                    )
                    .u32();
            }
        }
        if converted != 0 {
            let attachment = fn_00772820(e, Ptr::new(converted)).addr();
            let name = if attachment == 0 {
                0x0101_1584
            } else {
                attachment
            };
            e.call(
                0x0040_6d00,
                &args![0x011d_98d8_u32, 0x104_u32, 0x0101_9f08_u32, name],
            );
            let ammo_label = e.mem.u32(this.addr() + 0x15c);
            hud_set_text(e, ammo_label, 0x011d_98d8);
            let shifted = e.mem.u8(0x011d_96b8) != 0;
            if attachment == 0 && shifted {
                fn_0077f890(e, this, 0x32);
                e.mem.set_u8(0x011d_96b8, 0);
            } else if attachment != 0 && !shifted {
                fn_0077f890(e, this, -0x32);
                e.mem.set_u8(0x011d_96b8, 1);
            }
        }
        let modification = e.call(0x004b_da70, &args![item_change, 6_u32]).bool();
        let rate = tes_object_weap_get_ammo_regen_rate(e, Ptr::new(weapon), modification);
        let mut subtract_loaded = true;
        if rate > e.global::<f64>(0x0101_2060) {
            let modification = e.call(0x004b_da70, &args![item_change, 2_u32]).bool();
            reserve = e.call(0x004f_e160, &args![weapon, modification]).i32() as f32;
            subtract_loaded = false;
        }
        let player = e.mem.u32(0x011d_ea3c);
        let ammo = e.call(0x0052_5980, &args![weapon, player]).u32();
        if ammo != 0 && acquired != 0 {
            loaded = e.call(0x0072_6070, &args![acquired]).i32() as f32;
            if subtract_loaded {
                let player = e.mem.u32(0x011d_ea3c);
                let ammo = e.call(0x0052_5980, &args![weapon, player]).u32();
                let player = e.mem.u32(0x011d_ea3c);
                let count = inventory_count_for(e, player, ammo);
                reserve = (f64::from(count) - f64::from(loaded)) as f32;
            }
        } else if e.call(0x0047_4a80, &args![weapon + 0xa4]).bool() {
            let player = e.mem.u32(0x011d_ea3c);
            loaded = e.call(0x0096_9040, &args![player]).i32() as f32;
            reserve = loaded;
        }
    }
    if e.call(0x0064_50c0, &args![weapon]).bool() {
        let ammo_count_tile = e.mem.u32(this.addr() + 0x58);
        hud_trait(e, ammo_count_tile, 0xfa3, 0);
    } else if loaded != e.mem.f32(0x011a_011c)
        || reserve != e.mem.f32(0x011a_0118)
        || weapon != e.mem.u32(0x011d_98d4)
    {
        let reserve_int = x87_to_int(e, f64::from(reserve));
        let loaded_int = x87_to_int(e, f64::from(loaded));
        e.call(
            0x0040_6d00,
            &args![
                0x011d_97d0_u32,
                0x104_u32,
                0x0107_14ec_u32,
                loaded_int,
                reserve_int
            ],
        );
        let ammo_count_tile = e.mem.u32(this.addr() + 0x58);
        hud_set_text(e, ammo_count_tile, 0x011d_97d0);
        let ammo_count_tile = e.mem.u32(this.addr() + 0x58);
        hud_trait(e, ammo_count_tile, 0xfa3, 1);
        e.mem.set_f32(0x011a_011c, loaded);
        e.mem.set_f32(0x011a_0118, reserve);
    }
    if e.mem.u32(0x011d_98d4) != weapon {
        for offset in [0x4c_u32, 0x50, 0x54, 0x180] {
            let item = e.mem.u32(this.addr() + offset);
            hud_trait(e, item, 0xfa3, 1);
        }
        e.mem.set_u32(0x011d_98d4, weapon);
        if e.mem.u8(0x011d_96b8) != 0 {
            fn_0077f890(e, this, 0x32);
        }
        e.mem.set_u8(0x011d_96b8, 0);
    }
    let _first_health = e.call(0x004b_cdb0, &args![item_change, 0_u32]).f32();
    let condition = e.call(0x004b_cdb0, &args![item_change, 1_u32]).f32();
    let warning_limit: f64 = e.global(0x0104_f2f0);
    if f64::from(condition) < warning_limit {
        e.call(0x0053_30e0, &args![0x011d_9ae4_u32, 0.0_f32, alpha]);
        let current_alpha = e.mem.f32(0x011d_9ae4);
        if current_alpha == 0.0 || current_alpha == alpha {
            let old = e.mem.u8(0x011a_0114);
            e.mem.set_u8(0x011a_0114, u8::from(old == 0));
        }
        let scale: f64 = e.global(0x0102_0758);
        let maximum: f64 = e.global(0x0101_e568);
        let change = f64::from(alpha) / maximum * scale;
        let next = if e.mem.u8(0x011a_0114) != 0 {
            (f64::from(current_alpha) - change) as f32
        } else {
            (f64::from(current_alpha) + change) as f32
        };
        e.mem.set_f32(0x011d_9ae4, next);
    } else {
        e.mem.set_f32(0x011d_9ae4, alpha);
    }
    let condition_tile = e.mem.u32(this.addr() + 0x50);
    let condition_alpha = e.mem.f32(0x011d_9ae4);
    hud_set_float(e, condition_tile, 0xfa9, condition_alpha);
    if condition != e.mem.f32(0x011a_0110) {
        let health = e.call(0x004b_cdb0, &args![item_change, 0_u32]).f32();
        let has_mod = e.call(0x004b_da70, &args![item_change, 10_u32]).bool();
        let maximum = e.call(0x004b_cf00, &args![weapon, has_mod]).i32() as f32;
        let ratio = (f64::from(health) / f64::from(maximum)) as f32;
        let ratio = e.call(0x0040_4010, &args![0.0_f32, ratio]).f32();
        let ratio = e.call(0x0040_ebd0, &args![ratio, 1.0_f32]).f32();
        let offset = if e.mem.u8(0x011d_96b8) != 0 { -50 } else { 0 };
        let health_tile = e.mem.u32(this.addr() + 0x50);
        hud_main_menu_update_meter(e, Ptr::new(health_tile), ratio, 0, 0, offset);
        e.mem.set_f32(0x011a_0110, condition);
    }
}

// Translated from 007739f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::UpdateStatusModeUI` (Xbox PDB): sets the five status
/// labels, counts fully damaged body parts, and updates the condition icons.
pub fn hud_main_menu_update_status_mode_ui(
    e: &mut Engine,
    this: Ptr<HUDMainMenu>,
    status_mode: bool,
) {
    let menu = e.mem.u32(0x011d_96c0);
    for (setting, offset) in [
        (0x011d_4d74_u32, 0x17c_u32),
        (0x011d_3e74, 0x178),
        (0x011d_1f08, 0x164),
        (0x011d_37c8, 0x16c),
        (0x011d_246c, 0x168),
    ] {
        let label = e.call(0x0040_3df0, &args![setting]).u32();
        let target = e.mem.u32(menu + offset);
        hud_set_text(e, target, label);
    }
    let player = e.mem.u32(0x011d_ea3c);
    let values = if player == 0 { 0 } else { player + 0xa4 };
    let mut destroyed = 0_u8;
    for part in [0x1b_u32, 0x1d, 0x1e, 0x1c, 0x19, 0x1a] {
        let condition = e.call(0x0064_6800, &args![values, part, 0_u32]).f32();
        if condition == 0.0 {
            destroyed = destroyed.wrapping_add(1);
        }
    }
    let condition = if destroyed == 0 {
        0.0
    } else {
        let setting = match destroyed {
            1 => 0x011d_05a4,
            2 => 0x011d_0c84,
            3 => 0x011d_132c,
            _ => 0x011d_07fc,
        };
        let ptr = e.call(0x0040_3e20, &args![setting]).u32();
        (f64::from(e.mem.f32(ptr)) + 1.0) as f32
    };
    // GetActorValue takes only the id; the word the exe pushes before it
    // (00773cf2, 00773dc0, 00773dff, 00773e3e) is 00773e90's blink flag.
    let actor_value = |e: &mut Engine, id: u32| e.vcall(values, 8, &args![id]).i32() as f32;
    let radiation = actor_value(e, 0x36);
    let radiation_tile = e.mem.u32(menu + 0x178);
    fn_00773e90(e, this, Ptr::new(radiation_tile), radiation, false);
    let damaged_limb_tile = e.mem.u32(menu + 0x17c);
    fn_00773e90(
        e,
        this,
        Ptr::new(damaged_limb_tile),
        condition,
        !status_mode,
    );
    if !status_mode {
        for offset in [0x164_u32, 0x168, 0x16c] {
            let target = e.mem.u32(menu + offset);
            hud_set_float(e, target, 0xfa9, 0.0);
        }
    } else {
        for (offset, id, advance_blink) in [
            (0x164_u32, 0x49_u32, false),
            (0x168, 0x4b, false),
            (0x16c, 0x4a, true),
        ] {
            let value = actor_value(e, id);
            let item = e.mem.u32(menu + offset);
            fn_00773e90(e, this, Ptr::new(item), value, advance_blink);
        }
    }
}

// Translated from 00775a00 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::SetInfoForRef` (Xbox PDB): updates target name, lock/action
/// prompts, item details, button hints, and their tile visibility in PC order.
// The PC function installs an SEH frame at entry; exception unwinding is not translated.
// Translation incomplete: 00775a00 - compiler-generated SEH unwinding is omitted.
pub fn hud_main_menu_set_info_for_ref(
    e: &mut Engine,
    mut reference: Ptr,
    _unused_2: u32,
    refresh: u8,
) -> bool {
    let menu = current_hud(e);
    let player = current_player(e);
    if menu == 0 || player == 0 {
        return false;
    }
    if reference.addr() == player {
        reference = Ptr::new(0);
    }
    if refresh == 0 && reference.addr() == e.mem.u32(menu + 0x1b8) {
        if e.mem.u8(0x011d_96ba) != 0 && e.mem.u8(0x011d_96b9) != 0 {
            set_info_target_string(e);
            let target_tile = hud_field(e, 0xf4);
            hud_trait(e, target_tile, 0xff4, 2);
        } else {
            let reset = if reference.is_null() {
                true
            } else if e.vcall(reference.addr(), 0x100, &args![]).bool() {
                let player = current_player(e);
                !e.call(0x008b_c700, &args![reference, player]).bool()
            } else {
                false
            };
            if reset {
                if reference.is_null() {
                    set_info_target_string(e);
                    let target_tile = hud_field(e, 0xf4);
                    hud_trait(e, target_tile, 0xff4, 1);
                }
                e.mem.set_u8(0x011d_96bb, 0);
                if e.call(0x0045_8030, &args![]).u32() == 2 {
                    let target_tile = hud_field(e, 0x120);
                    hud_trait(e, target_tile, 0xfa3, 1);
                }
            }
        }
        return false;
    }

    if reference.is_null() {
        let player = current_player(e);
        e.call(0x0096_3e00, &args![player, 0xc_u32, 0_u32]);
        let player = current_player(e);
        e.call(0x0096_3e00, &args![player, 0xb_u32, 0_u32]);
        let menu = current_hud(e);
        e.mem.set_u32(menu + 0x1b8, 0);
        return clear_info_target_tiles(e);
    }

    let actor = e.vcall(reference.addr(), 0x100, &args![]).bool();
    let terminal_base = e.call(0x007a_f430, &args![reference]).u32();
    let form = e.call(0x007a_f430, &args![reference]).u32();
    let terminal = e.call(0x0040_1170, &args![form]).u32() == 0x17;
    if reference.addr() == e.mem.u32(current_hud(e) + 0x1b8) {
        let player = current_player(e);
        e.call(0x0096_3e00, &args![player, 0xc_u32, 0_u32]);
        let player = current_player(e);
        e.call(0x0096_3e00, &args![player, 0xb_u32, 0_u32]);
    } else if actor {
        let player = current_player(e);
        if e.call(0x0049_97b0, &args![player]).bool() {
            if !e.call(0x0056_6950, &args![reference]).bool() {
                set_info_player_action(e, 0xc);
            }
        } else {
            let player = current_player(e);
            e.call(0x0096_3e00, &args![player, 0xc_u32, 0_u32]);
            let player = current_player(e);
            e.call(0x0096_3e00, &args![player, 0xb_u32, 0_u32]);
        }
    } else {
        let lock = e.call(0x0056_9160, &args![reference]).u32();
        if terminal {
            if !e.call(0x0050_1ae0, &args![terminal_base, reference]).bool() {
                let owner = e.call(0x0056_7790, &args![reference]).u32();
                if owner != 0 {
                    set_info_player_action(e, 0xb);
                }
            }
        } else {
            let base = e.call(0x007a_f430, &args![reference]).u32();
            let form_type = e.call(0x0040_1170, &args![base]).u32();
            if form_type == 0x1b {
                let owner = e.call(0x0056_7790, &args![reference]).u32();
                if owner != 0 {
                    if e.call(0x0057_5590, &args![reference, 0_u32, 1_u32]).i32() > 0
                        && lock != 0
                        && e.call(0x0050_21a0, &args![lock]).bool()
                        && (e.mem.u32(lock + 4) == 0 || {
                            let key = e.mem.u32(lock + 4);
                            let player = current_player(e);
                            e.call(0x0057_5610, &args![player, key]).i32() < 1
                        })
                    {
                        set_info_player_action(e, 0xb);
                    }
                } else {
                    set_info_locked_owner_action(e, reference, lock);
                }
            } else {
                set_info_locked_owner_action(e, reference, lock);
            }
        }
    }
    if refresh == 0 {
        let menu = current_hud(e);
        e.mem.set_u32(menu + 0x1b8, reference.addr());
    }
    if e.call(0x0047_7ba0, &args![reference]).bool() && !actor {
        return clear_info_target_tiles(e);
    }
    let target_type = e.call(0x0057_9280, &args![reference]).u32();
    if target_type == 3 || target_type == 5 {
        let marker_found = e.with_stack(0x20, |e, marker| {
            e.mem.set_u32(marker.addr(), 0);
            e.call(0x0050_9890, &args![marker.addr() + 4, 0xff_u32]);
            let player = current_player(e);
            // 00776044: slot 0x1f4 takes no stack words; the five pushed words
            // are the rest of `005686b0` (RET 0x18).
            let location = e.vcall(player, 0x1f4, &args![]).u32();
            e.call(
                0x0056_86b0,
                &args![
                    reference,
                    location,
                    1_u32,
                    1_u32,
                    marker.addr() + 4,
                    marker,
                    1_u32
                ],
            )
            .bool()
        });
        if !marker_found {
            return false;
        }
    }

    let player = current_player(e);
    let perceived = if e.call(0x0096_7090, &args![player, reference]).u32() != 0 {
        let player = current_player(e);
        let perception = e.call(0x0096_7090, &args![player, reference]).u32();
        e.mem.u8(perception + 4) != 0
    } else {
        false
    };
    if target_type == 0 && !perceived {
        set_info_target_prompt(e, 1, 0x1004);
    } else if !e.call(0x0057_9690, &args![reference]).bool()
        || (actor && e.call(0x0056_6950, &args![reference]).bool())
    {
        if !perceived {
            if !actor || !e.call(0x0049_3bb0, &args![reference]).bool() {
                set_info_target_prompt(e, 1, 0x1005);
            } else {
                set_info_target_prompt(e, 1, 0x1004);
            }
        } else {
            set_info_target_prompt(e, 2, 0x1004);
        }
    } else {
        set_info_target_prompt(e, 2, 0x1005);
    }

    let form = e.call(0x007a_f430, &args![reference]).u32();
    let form_type = e.call(0x0040_1170, &args![form]).u32();
    let mut display_mask = 0_u32;
    let mut _alchemy_form = 0_u32;
    let mut armor_form = 0_u32;
    let mut weapon_form = 0_u32;
    match form_type {
        0x15 | 0x16 | 0x17 | 0x1b | 0x1c | 0x27 | 0x2a | 0x2b => {}
        0x18 => {
            _alchemy_form = e.call(0x007a_f430, &args![reference]).u32();
            display_mask |= 0xfc;
        }
        0x1e => {
            set_info_target_prompt(e, 1, 0x1004);
            let form = e.call(0x007a_f430, &args![reference]).u32();
            if !e.call(0x0046_f070, &args![form]).bool() {
                return true;
            }
            display_mask |= 0xfc;
        }
        0x20 | 0x21 | 0x25 => return set_info_unsupported_form(e),
        0x28 => {
            armor_form = e.call(0x007a_f430, &args![reference]).u32();
            if e.call(0x0047_bcf0, &args![armor_form]).bool() {
                display_mask |= 0xfc;
            }
        }
        0x29 => {
            weapon_form = e.call(0x007a_f430, &args![reference]).u32();
            if e.call(0x004c_94d0, &args![weapon_form]).bool() {
                display_mask |= 0xfc;
            }
        }
        0x33 => {
            let nested = e.call(0x009d_9f00, &args![reference]).u32();
            let mut inspect_direct_form = nested == 0;
            if nested != 0 {
                let nested = e.call(0x009d_9f00, &args![reference]).u32();
                let form = e.call(0x0044_ddc0, &args![nested]).u32();
                inspect_direct_form = e.call(0x0040_1170, &args![form]).u32() == 0x1f;
            }
            if inspect_direct_form {
                if e.call(0x0040_1170, &args![reference]).u32() == 0x3d {
                    return set_info_unsupported_form(e);
                }
                display_mask |= 0xfc;
            }
        }
        0x1f | 0x23 | 0x2f | 0x67 => display_mask |= 0xfc,
        _ => {
            let target_tile = hud_field(e, 0xf4);
            hud_trait(e, target_tile, 0xff4, 1);
        }
    }

    let teleport = e.call(0x0056_8e50, &args![reference]).u32();
    let lock = e.call(0x0056_9160, &args![reference]).u32();
    let _lock_base = e.call(0x007a_f430, &args![reference]).u32();
    let _second_base = e.call(0x007a_f430, &args![reference]).u32();
    let can_ignore_lock = if form_type == 0x1c {
        let player = current_player(e);
        e.call(0x0051_8f00, &args![reference, player, 0_u32, 1_u32])
            .bool()
    } else if form_type == 0x1b {
        if lock == 0 || !e.call(0x0050_21a0, &args![lock]).bool() {
            true
        } else if e.mem.u32(lock + 4) != 0 {
            let key = e.mem.u32(lock + 4);
            e.with_stack(4, |e, found| {
                e.mem.set_u32(found.addr(), 0);
                let player = current_player(e);
                e.call(0x0089_1db0, &args![player, key, 0_u32, 1_u32, 0_u32, found])
                    .bool()
            })
        } else {
            false
        }
    } else {
        true
    };
    let terminal_level = if terminal {
        e.call(0x0050_11a0, &args![terminal_base, reference]).u32()
    } else {
        0
    };
    let player = current_player(e);
    let first_access = e.call(0x0096_6c60, &args![player, reference]).u32();
    let access = if first_access == 2 {
        true
    } else {
        let player = current_player(e);
        let second_access = e.call(0x0096_6c60, &args![player, reference]).u32();
        if second_access == 3 {
            true
        } else {
            let player = current_player(e);
            e.call(0x0096_6c60, &args![player, reference]).u32() == 4
        }
    };
    let hide_lock = (!terminal || !access)
        && !e.call(0x0057_b460, &args![reference]).bool()
        && (lock == 0 || !e.call(0x0050_21a0, &args![lock]).bool());
    if hide_lock {
        let lock_tile = hud_field(e, 0xac);
        hud_trait(e, lock_tile, 0xfa9, 0);
    } else {
        let raw_level = if terminal {
            terminal_level
        } else {
            let numeric = e.call(0x0056_91b0, &args![reference]).u32();
            e.call(0x0043_0b40, &args![numeric]).u32()
        };
        let clamped = e.call(0x0064_7b70, &args![raw_level, 0_u32]).u32();
        let level = e.call(0x004a_8f20, &args![clamped, 5_u32]).u32();
        let has_key = if lock != 0 && e.mem.u32(lock + 4) != 0 {
            let key = e.mem.u32(lock + 4);
            e.with_stack(4, |e, found| {
                e.mem.set_u32(found.addr(), 0);
                let player = current_player(e);
                e.call(0x0089_1db0, &args![player, key, 0_u32, 1_u32, 0_u32, found])
                    .bool()
            })
        } else {
            false
        };
        if lock != 0 && e.call(0x0043_0ae0, &args![lock]).bool() {
            set_info_lock_text_alpha(e, 0x011d_2f28);
        } else if e.call(0x0057_b460, &args![reference]).bool() {
            set_info_lock_text_alpha(e, 0x011d_44d4);
        } else {
            let lock_form = form_type == 0x1b || form_type == 0x1c;
            if (lock_form && (!can_ignore_lock || has_key)) || form_type == 0x17 {
                if has_key {
                    let key_text = e.call(0x0040_3df0, &args![0x011d_3be0_u32]).u32();
                    let lock_text = e.call(0x0040_3df0, &args![0x011d_2454_u32]).u32();
                    e.call(
                        0x0040_6d00,
                        &args![
                            0x011d_9d50_u32,
                            0x104_u32,
                            0x0107_34ec_u32,
                            lock_text,
                            key_text
                        ],
                    );
                } else if level == 5 {
                    e.call(
                        0x0040_6d30,
                        &args![0x011d_9d50_u32, 0x104_u32, 0x0106_3e28_u32],
                    );
                    let setting = e.mem.u32(0x0118_4a98 + level * 4);
                    let name = e.call(0x0040_3df0, &args![setting]).u32();
                    e.call(0x0040_6d50, &args![0x011d_9d50_u32, 0x104_u32, name]);
                    e.call(
                        0x0040_6d50,
                        &args![0x011d_9d50_u32, 0x104_u32, 0x0106_3e18_u32],
                    );
                } else {
                    let setting = e.mem.u32(0x0118_4a98 + level * 4);
                    let name = e.call(0x0040_3df0, &args![setting]).u32();
                    let lock_text = e.call(0x0040_3df0, &args![0x011d_2454_u32]).u32();
                    e.call(
                        0x0040_6d00,
                        &args![0x011d_9d50_u32, 0x104_u32, 0x0107_34ec_u32, lock_text, name],
                    );
                }
                let lock_tile = hud_field(e, 0xac);
                hud_set_text(e, lock_tile, 0x011d_9d50);
                set_info_lock_alpha(e);
            } else {
                let lock_tile = hud_field(e, 0xac);
                hud_trait(e, lock_tile, 0xfa9, 0);
            }
        }
    }

    let extra = e.call(0x005d_43c0, &args![reference]).u32();
    let ash_pile = e.call(0x0041_e310, &args![extra]).u32();
    let empty_ash_pile = ash_pile != 0
        && e.call(0x0055_d310, &args![ash_pile]).u32() != 0
        && e.call(0x0057_5590, &args![ash_pile, 0_u32, 0_u32]).i32() < 1;
    let extra = e.call(0x005d_43c0, &args![reference]).u32();
    let blocked = e.call(0x0041_df00, &args![extra]).u32() != 0;
    let empty_container = !blocked
        && ((e.call(0x0055_d310, &args![reference]).u32() != 0
            && e.call(0x0057_5590, &args![reference, 0_u32, 0_u32]).i32() < 1)
            || empty_ash_pile)
        && (!actor || e.vcall(reference.addr(), 0x22c, &args![1_u32]).bool());
    if empty_container {
        set_info_tile_alpha(e, 0xb0);
    } else {
        let container_tile = hud_field(e, 0xb0);
        hud_trait(e, container_tile, 0xfa9, 0);
    }

    let player = current_player(e);
    let heading = e.vcall(player, 0x2bc, &args![0_u32]).f64();
    let degrees_per_radian: f64 = e.global(0x0102_f248);
    let _player_heading = quiet_f32((heading * degrees_per_radian) as f32);
    let mut _target_heading = 0.0_f32;
    if hud_field(e, 0x21c) != 0 {
        let target = hud_field(e, 0x21c);
        let location = e.vcall(target, 0x1f4, &args![]).u32();
        let player = current_player(e);
        let player_location = e.vcall(player, 0x1f4, &args![]).u32();
        _target_heading =
            quiet_f32(e.call(0x0077_2840, &args![player_location, location]).f64() as f32);
    }
    let highlighted_target = if hud_field(e, 0x21c) == 0 {
        false
    } else {
        let target = hud_field(e, 0x21c);
        !e.vcall(target, 0x22c, &args![0_u32]).bool()
            && {
                let target = hud_field(e, 0x21c);
                !e.call(0x0047_7ba0, &args![target]).bool()
            }
            && e.mem.u8(0x011d_96c5) != 0
            && hud_field(e, 0x21c) == reference.addr()
    };
    if e.vcall(reference.addr(), 0x224, &args![]).bool() {
        let nested = e.call(0x009d_9f00, &args![reference]).u32();
        if nested != 0 {
            let nested = e.call(0x009d_9f00, &args![reference]).u32();
            let form = e.call(0x0044_ddc0, &args![nested]).u32();
            let name = e.call(0x0048_2720, &args![form]).u32();
            e.call(0x0040_6d30, &args![0x011d_9c48_u32, 0x104_u32, name]);
        } else {
            let direct = e.call(0x0047_6c90, &args![reference]).u32();
            if direct != 0 {
                let mut form = e.call(0x0047_6c90, &args![reference]).u32();
                if e.call(0x0052_5a90, &args![form, 0_u32]).u32() != 0 {
                    let candidate = e.call(0x0052_5a90, &args![form, 0_u32]).u32();
                    if e.call(0x004f_d400, &args![candidate]).u32() != 0 {
                        let candidate = e.call(0x0052_5a90, &args![form, 0_u32]).u32();
                        form = e.call(0x004f_d400, &args![candidate]).u32();
                    }
                }
                let name = e.call(0x0048_2720, &args![form]).u32();
                e.call(0x0040_6d30, &args![0x011d_9c48_u32, 0x104_u32, name]);
            } else {
                let name = e.call(0x0055_d520, &args![reference]).u32();
                e.call(0x0040_6d30, &args![0x011d_9c48_u32, 0x104_u32, name]);
            }
        }
    } else {
        let name = e.call(0x0055_d520, &args![reference]).u32();
        e.call(0x0040_6d30, &args![0x011d_9c48_u32, 0x104_u32, name]);
    }
    let player = current_player(e);
    let actors_in_combat = e.call(0x0095_3c20, &args![player]).i32() > 0;
    if e.mem.u8(0x011d_9c48) == 0 {
        let name_tile = hud_field(e, 0xa8);
        hud_set_text(e, name_tile, 0x0102_0770);
    } else if teleport == 0 {
        let actor_name_allowed = if !actor {
            true
        } else {
            let out_of_combat = if reference.addr() != hud_field(e, 0x21c) {
                let player = current_player(e);
                !e.call(0x008b_c700, &args![reference, player]).bool()
            } else {
                false
            };
            out_of_combat
                || !actors_in_combat
                || e.vcall(reference.addr(), 0x22c, &args![0_u32]).bool()
        };
        if actor_name_allowed && !highlighted_target {
            let extra = e.call(0x005d_43c0, &args![reference]).u32();
            let count = e.call(0x0041_8770, &args![extra]).u32() as u16 as i16;
            let mut displayed = true;
            if count == 1 {
                if actor {
                    let name_tile = hud_field(e, 0xa8);
                    hud_set_text(e, name_tile, 0x011d_9c48);
                } else if (weapon_form == 0 || !e.call(0x004c_94d0, &args![weapon_form]).bool())
                    && (armor_form == 0 || !e.call(0x0047_bcf0, &args![armor_form]).bool())
                    && (armor_form != 0 || weapon_form != 0)
                {
                    displayed = false;
                } else {
                    let form = e.call(0x007a_f430, &args![reference]).u32();
                    let form_type = e.call(0x0040_1170, &args![form]).u32();
                    let stack_count = if form_type == 0x29 {
                        let form = e.call(0x007a_f430, &args![reference]).u32();
                        e.call(0x0040_1170, &args![form + 0x80]).i32()
                    } else {
                        0
                    };
                    if stack_count > 1 {
                        let quantity = e.call(0x0077_8930, &args![reference]).u32();
                        e.with_stack(0x104, |e, buffer| {
                            e.call(
                                0x0040_6d00,
                                &args![
                                    buffer,
                                    0x104_u32,
                                    0x0106_eb00_u32,
                                    0x011d_9c48_u32,
                                    quantity
                                ],
                            );
                            let name_tile = hud_field(e, 0xa8);
                            hud_set_text(e, name_tile, buffer.addr());
                        });
                    } else {
                        let name_tile = hud_field(e, 0xa8);
                        hud_set_text(e, name_tile, 0x011d_9c48);
                    }
                }
            } else {
                let extra = e.call(0x005d_43c0, &args![reference]).u32();
                let count = e.call(0x0041_8770, &args![extra]).u32() as u16 as i16;
                e.with_stack(0x104, |e, buffer| {
                    e.call(
                        0x0040_6d00,
                        &args![
                            buffer,
                            0x104_u32,
                            0x0106_eb00_u32,
                            0x011d_9c48_u32,
                            count as i32
                        ],
                    );
                    let name_tile = hud_field(e, 0xa8);
                    hud_set_text(e, name_tile, buffer.addr());
                });
            }
            for offset in [0x7c_u32, 0x80, 0x84] {
                let tile = hud_field(e, offset);
                hud_trait(e, tile, 0xfa3, 0);
            }
            if displayed {
                display_mask |= 4;
            }
        }
    } else {
        // DoorTeleportData::GetTeleportCellMapName writes an 8-byte string
        // temporary. The PC body formats the door name, localized separator,
        // and destination into the fixed 0x104-byte stack buffer.
        set_info_teleport_name(e, teleport);
        display_mask |= 4;
    }

    let inventory_target_type = e.call(0x0057_9280, &args![reference]).u32();
    let mut marked_actor = false;
    if actor {
        let player = current_player(e);
        let mut node = e.call(0x0059_5d10, &args![player]).u32();
        while node != 0 && !e.call(0x0082_56d0, &args![node]).bool() {
            let slot = e.call(0x0068_15c0, &args![node]).u32();
            let entry = e.mem.u32(slot);
            if e.mem.u32(entry) == reference.addr() && e.mem.u8(entry + 4) != 0 {
                marked_actor = true;
                break;
            }
            node = e.call(0x0072_6070, &args![node]).u32();
        }
    }
    if !highlighted_target {
        let actor_inventory_allowed = if inventory_target_type != 7 || !actor {
            true
        } else {
            let out_of_combat = if !marked_actor && reference.addr() != hud_field(e, 0x21c) {
                let player = current_player(e);
                !e.call(0x008b_c700, &args![reference, player]).bool()
            } else {
                false
            };
            out_of_combat
                || (!e.vcall(reference.addr(), 0x274, &args![0_u32]).bool()
                    && !e.call(0x0089_4900, &args![reference]).bool()
                    && !marked_actor)
        };
        if actor_inventory_allowed && !e.call(0x0047_7ba0, &args![reference]).bool() {
            let inventory = e.call(0x0052_7080, &args![reference, 0x53_u32]).u32();
            set_info_inventory_action(
                e,
                reference,
                inventory,
                inventory_target_type,
                form_type,
                actor,
                lock,
                can_ignore_lock,
                &mut display_mask,
            );
        }
    }
    set_info_item_details(e, reference, actor, marked_actor, display_mask)
}

// Translated from 00778930 (decompiled, FalloutNV.exe 1.4.0.525)
/// Gets the count of an extra-data list reached through the input object.
pub fn fn_00778930(e: &mut Engine, object: Ptr) -> i32 {
    let extra_list = e.call(0x005d_43c0, &args![object]).u32();
    e.call(0x0041_8770, &args![extra_list]).i32() as i16 as i32
}

// Translated from 00778950 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the HUD's seen-level float as its raw stored word.
pub fn fn_00778950(e: &mut Engine, this: Ptr<HUDMainMenu>) -> u32 {
    e.mem.u32(this.addr() + 0x188)
}

// Translated from 00778970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the word immediately after the HUD's seen-level float.
pub fn fn_00778970(e: &mut Engine, this: Ptr<HUDMainMenu>) -> u32 {
    e.mem.u32(this.addr() + 0x18c)
}

// Translated from 00778990 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::DisplayWaterRadiation` (Xbox PDB): formats the adjusted
/// radiation amount for one HUD tile, then sets a label on another tile.
pub fn hud_main_menu_display_water_radiation(
    e: &mut Engine,
    radiation: i32,
    amount_tile_id: u32,
    label_tile_id: u32,
) {
    let player = current_player(e);
    let resistance = e.call(0x008c_4330, &args![player]).f32();
    e.with_stack(4, |e, multiplier| {
        e.mem.set_f32(multiplier.addr(), 1.0);
        let player = current_player(e);
        e.call(0x005e_58f0, &args![0x1a_u32, player, multiplier]);
        let resistance = (f64::from(resistance) * f64::from(e.mem.f32(multiplier.addr()))) as f32;
        let amount = (f64::from(radiation) * f64::from(resistance)) as f32;
        let zero: f64 = e.global(0x0101_2060);
        let one: f64 = e.global(0x0101_2070);
        let displayed = if f64::from(amount) > zero && f64::from(amount) < one {
            1
        } else {
            e.call(0x00ec_62c0, &args![f64::from(amount)]).i32()
        };
        e.with_stack(0x108, |e, buffer| {
            e.call(0x00ec_623a, &args![buffer, 0x0102_0774_u32, displayed]);
            let tile = hud_tile(e, amount_tile_id);
            hud_set_text(e, tile, buffer.addr());
        });
    });
    let label = e.call(0x0040_3df0, &args![0x011d_30c0_u32]).u32();
    let tile = hud_tile(e, label_tile_id);
    hud_set_text(e, tile, label);
}

// Translated from 00778a90 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::DisplayWaterHealth` (Xbox PDB): formats the health amount
/// for one HUD tile, then sets a label on another tile.
pub fn hud_main_menu_display_water_health(
    e: &mut Engine,
    health: i32,
    amount_tile_id: u32,
    label_tile_id: u32,
) {
    e.with_stack(0x10c, |e, buffer| {
        e.call(0x00ec_623a, &args![buffer, 0x0102_0774_u32, health]);
        let tile = hud_tile(e, amount_tile_id);
        hud_set_text(e, tile, buffer.addr());
    });
    let label = e.call(0x0040_3df0, &args![0x011d_300c_u32]).u32();
    let tile = hud_tile(e, label_tile_id);
    hud_set_text(e, tile, label);
}

// Translated from 00778b10 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::RefreshInfoForRef` (Xbox PDB): rebuilds the HUD's current
/// reference information when a HUD instance exists.
pub fn hud_main_menu_refresh_info_for_ref(e: &mut Engine) {
    if current_hud(e) != 0 {
        let reference = hud_field(e, 0x1b8);
        hud_main_menu_set_info_for_ref(e, Ptr::new(reference), 0, 1);
    }
}

// Translated from 00778b40 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the HUD's last reference only when it equals `reference`.
pub fn fn_00778b40(e: &mut Engine, reference: Ptr) -> bool {
    let hud = current_hud(e);
    if hud != 0 && e.mem.u32(hud + 0x1b8) == reference.addr() {
        e.mem.set_u32(hud + 0x1b8, 0);
        true
    } else {
        false
    }
}

// Translated from 00778b80 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::GetCompassAngle` (Xbox PDB): takes two 3-float vectors by
/// value, subtracts them through `00439ef0`, and converts their relative
/// heading to the compass's degree scale.
#[allow(clippy::too_many_arguments)]
pub fn hud_main_menu_get_compass_angle(
    e: &mut Engine,
    first_x: f32,
    first_y: f32,
    first_z: f32,
    second_x: f32,
    second_y: f32,
    second_z: f32,
) -> f64 {
    e.with_stack(24, |e, vectors| {
        for (index, value) in [first_x, first_y, first_z, second_x, second_y, second_z]
            .into_iter()
            .enumerate()
        {
            e.mem.set_f32(vectors.addr() + index as u32 * 4, value);
        }
        e.with_stack(12, |e, difference| {
            e.call(
                0x0043_9ef0,
                &args![vectors, difference, vectors.addr() + 12],
            );
            let direction = e.call(0x004b_1550, &args![difference]).f64() as f32;
            let player = current_player(e);
            let heading = e.vcall(player, 0x2bc, &args![0_u32]).f64();
            let mut angle = (f64::from(direction) - heading) as f32;
            if f64::from(angle) <= e.global::<f64>(0x0101_ff58) {
                angle = (f64::from(angle) + e.global::<f64>(0x0101_ff48)) as f32;
            } else if f64::from(angle) > e.global::<f64>(0x0101_ff40) {
                angle = (e.global::<f64>(0x0101_ff48) - f64::from(angle)) as f32;
            }
            angle = (f64::from(angle) * e.global::<f64>(0x0102_f248)) as f32;
            angle = (f64::from(angle) * e.global::<f64>(0x0107_34f8)) as f32;
            f64::from(angle)
        })
    })
}

fn blink_setting(e: &mut Engine, setting: u32) -> i32 {
    let value = e.call(0x0043_d4d0, &args![setting]).u32();
    e.mem.i32(value)
}

fn blink_duration(e: &mut Engine, setting: u32) -> i32 {
    let value = blink_setting(e, setting);
    fn_0077f9c0(e, value)
}

fn blink_interpolate(e: &mut Engine, start: i32, end: i32, low: i32, high: i32, x: f32) -> f32 {
    e.call(
        0x004b_3ab0,
        &args![start as f32, end as f32, low as f32, high as f32, x],
    )
    .f32()
}

// Translated from 00778c20 (decompiled, FalloutNV.exe 1.4.0.525)
/// `BlinkTile` (Xbox PDB): advances one tile's alpha and blink state using
/// game settings, its `_AlphaDown` user trait, and the game clock.
// Translation incomplete: 00778c20 - compiler-generated SEH unwinding is omitted.
pub fn blink_tile(e: &mut Engine, tile: Ptr, mut period: f32, amplitude: f32) {
    let initialized = e.mem.u32(0x011d_9e58);
    if initialized & 1 == 0 {
        e.mem.set_u32(0x011d_9e58, initialized | 1);
        let alpha_down = e.call(0x00a0_0940, &args![0x0107_3500_u32, -1_i32]).u32();
        e.mem.set_u32(0x011d_9e54, alpha_down);
    }
    let maximum = blink_duration(e, 0x011d_1d54);
    let minimum = blink_duration(e, 0x011d_30d8);
    let breakpoint = blink_duration(e, 0x011d_4d44);
    if period > maximum as f32 {
        period = maximum as f32;
    } else if period < minimum as f32 {
        period = minimum as f32;
    }
    let below_breakpoint = period < breakpoint as f32;
    let alpha_down = e.mem.u32(0x011d_9e54);
    let state = e.call(0x00a0_11b0, &args![tile, alpha_down]).f32();
    let timing = if below_breakpoint {
        let start = blink_setting(e, 0x011d_3c94);
        let end = blink_setting(e, 0x011d_30f0);
        blink_interpolate(e, start, end, breakpoint, minimum, period)
    } else {
        let start = blink_setting(e, 0x011d_2bd4);
        let end = blink_setting(e, 0x011d_3c94);
        blink_interpolate(e, start, end, maximum, breakpoint, period)
    };
    let max_alpha = e.mem.f32(0x011d_979c);
    let mut alpha = 0.0_f32;
    if state == 0.0 || state == 1.0 {
        if max_alpha != 0.0 {
            let current = e.call(0x00a0_11b0, &args![tile, 0xfa9_u32]).f64();
            let factor = f64::from(timing) * f64::from(amplitude) / f64::from(max_alpha);
            let elapsed = e.call(0x0084_d030, &args![0x011f_6394_u32]).f64();
            alpha = if state == 0.0 {
                (elapsed * factor + current) as f32
            } else {
                (current - elapsed * factor) as f32
            };
        }
    } else if state == e.global::<f64>(0x0102_1928) as f32 {
        alpha = max_alpha;
    }
    e.call(0x00a0_12d0, &args![tile, 0xfa9_u32, alpha, 1_u32]);
    let current_alpha = e.call(0x00a0_11b0, &args![tile, 0xfa9_u32]).f32();
    if current_alpha > amplitude && !current_alpha.is_nan() && !amplitude.is_nan() {
        e.call(0x00a0_12d0, &args![tile, 0xfa9_u32, amplitude, 1_u32]);
        hud_trait(e, tile.addr(), alpha_down, 3);
    } else if current_alpha < 0.0 {
        let delay = if below_breakpoint {
            let start = blink_setting(e, 0x011d_46cc);
            let end = blink_setting(e, 0x011d_3924);
            blink_interpolate(e, start, end, breakpoint, minimum, period)
        } else {
            let start = blink_setting(e, 0x011d_24f0);
            let end = blink_setting(e, 0x011d_46cc);
            blink_interpolate(e, start, end, maximum, breakpoint, period)
        };
        e.call(0x00a0_12d0, &args![tile, 0xfa9_u32, 0.0_f32, 1_u32]);
        hud_trait(e, tile.addr(), alpha_down, 0);
        // 00778fd4: `00825c00` takes no stack word (plain RET); the pushed 1 is the
        // last word of `00a012d0` (RET 0xc).
        let now = e.call(0x0082_5c00, &args![0x011f_6394_u32]).u32();
        let next = (f64::from(now) + f64::from(delay)) as f32;
        e.call(0x00a0_12d0, &args![tile, 0x100e_u32, next, 1_u32]);
    } else {
        let next = e.call(0x00a0_11b0, &args![tile, 0x100e_u32]).f64();
        let now = e.call(0x0082_5c00, &args![0x011f_6394_u32]).u32();
        if f64::from(now) > next && state == e.global::<f64>(0x0102_1928) as f32 {
            hud_trait(e, tile.addr(), alpha_down, 1);
        }
    }
}

fn compass_position(e: &mut Engine, object: u32) -> [f32; 3] {
    let position = e.vcall(object, 0x1f4, &args![]).u32();
    [
        e.mem.f32(position),
        e.mem.f32(position + 4),
        e.mem.f32(position + 8),
    ]
}

fn compass_distance_squared(e: &mut Engine, from: [f32; 3], to: [f32; 3]) -> f32 {
    e.with_stack(24, |e, points| {
        for (index, value) in from.into_iter().chain(to).enumerate() {
            e.mem.set_f32(points.addr() + index as u32 * 4, value);
        }
        e.with_stack(12, |e, difference| {
            let vector = e
                .call(0x0043_9ef0, &args![points, difference, points.addr() + 12])
                .u32();
            let squared = e.call(0x004a_7290, &args![vector]).f32();
            e.call(0x0040_8840, &args![squared]).f32()
        })
    })
}

fn compass_interpolate(e: &mut Engine, a: f32, b: f32, low: f32, high: f32, x: f32) -> f32 {
    e.call(0x004b_3ab0, &args![a, b, low, high, x]).f32()
}

fn compass_take_tile(e: &mut Engine, pool: Ptr, cursor: &mut u32) -> u32 {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), *cursor);
        let entry = e.call(0x0057_cbe0, &args![pool.addr() + 4, slot]).u32();
        *cursor = e.mem.u32(slot.addr());
        e.mem.u32(entry)
    })
}

fn compass_take_objective_tile(e: &mut Engine, pool: Ptr, cursor: &mut u32) -> u32 {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), *cursor);
        let entry = e.call(0x0068_3520, &args![pool.addr() + 4, slot]).u32();
        *cursor = e.mem.u32(slot.addr());
        e.mem.u32(entry)
    })
}

fn compass_set_texture(e: &mut Engine, tile: u32, texture: u32) {
    e.call(0x00a0_1350, &args![tile, 0xfcc_u32, texture, 1_u32]);
}

// Translated from 00779070 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::UpdateCompass` (Xbox PDB): initializes compass tile traits,
/// updates its heading offset and landmark, objective, player, and VATS markers.
// Translation incomplete: 00779070 - compiler-generated SEH unwinding is omitted.
pub fn hud_main_menu_update_compass(
    e: &mut Engine,
    compass: Ptr,
    landmarks: Ptr,
    objectives: Ptr,
    actor_ticks: Ptr,
) {
    for (bit, slot, name) in [
        (1_u32, 0x011d_9e64, 0x0107_3618),
        (2, 0x011d_9e60, 0x0107_360c),
        (4, 0x011d_9e5c, 0x0107_3500),
    ] {
        let flags = e.mem.u32(0x011d_9e68);
        if flags & bit == 0 {
            e.mem.set_u32(0x011d_9e68, flags | bit);
            let trait_id = e.call(0x00a0_0940, &args![name, -1_i32]).u32();
            e.mem.set_u32(slot, trait_id);
        }
    }
    let player = current_player(e);
    let player_heading = e.vcall(player, 0x2bc, &args![0_u32]).f64();
    let player = current_player(e);
    let process = e.call(0x008d_6f30, &args![player]).u32();
    let process_heading = e.call(0x0055_5ad0, &args![process]).f64();
    let degrees = ((process_heading + player_heading) * e.global::<f64>(0x0102_f248)) as f32;
    let magnitude = e.call(0x0040_8840, &args![degrees]).f64();
    let revolutions = e
        .call(
            0x00ec_62c0,
            &args![magnitude / e.global::<f64>(0x0103_c348)],
        )
        .i32();
    let offset = (f64::from(degrees) - f64::from(revolutions.wrapping_mul(360))) as f32;
    let semicircle: f32 = e.global(0x0107_3608);
    let normalized = e
        .call(
            0x004b_3ab0,
            &args![0.0_f32, 1.0_f32, 0.0_f32, semicircle, offset],
        )
        .f32();
    let compass_offset = (f64::from(normalized) - e.global::<f64>(0x0107_0c80)) as f32;
    let target = e.vcall(compass.addr(), 0x20, &args![]).u32();
    if target != 0 {
        let adjusted = if compass_offset < 0.0 {
            (f64::from(compass_offset) + e.global::<f64>(0x0101_2070)) as f32
        } else {
            compass_offset
        };
        let target = e.vcall(compass.addr(), 0x20, &args![]).u32();
        fn_0077a450(e, Ptr::new(target), adjusted, 0.0);
    }
    let player = current_player(e);
    let marker = e.call(0x0094_8030, &args![player]).u32();
    let player = current_player(e);
    let player_position = compass_position(e, player);
    let hud = current_hud(e);
    let display_width = e.mem.i32(hud + 0x214);
    let inner_width = (f64::from(display_width) * e.global::<f64>(0x0102_17d8) * 2.0) as f32;
    let outer_width = (f64::from(display_width) * 2.0) as f32;
    let opacity_setting = e.call(0x0044_ddc0, &args![0x011f_2250_u32]).u32();
    let opacity = if opacity_setting == 0 {
        let value = e.call(0x0040_3e20, &args![0x011d_9774_u32]).u32();
        e.mem.f32(value)
    } else {
        1.0
    };
    let max_alpha = (f64::from(opacity) * e.global::<f64>(0x0101_e568)) as f32;
    let landmarks_list = landmarks.addr() + 4;
    let mut landmark_tiles = e.call(0x0055_9450, &args![landmarks_list]).u32();
    let player = current_player(e);
    let sneaking = e.vcall(player, 0x22c, &args![0_u32]).bool();
    let landmark_limit = blink_setting(e, 0x011d_12cc);
    let landmark_limit_squared = landmark_limit.wrapping_mul(landmark_limit);
    // byte [EBP-0x8f] (state+5): written only on the not-sneaking path (00779591 = 1
    // when changed, 007795d6 = 0 for an unchanged travel marker) and otherwise kept
    // from the previous marker. Uninitialized in the exe before its first write.
    let mut location_changed: u8 = 0;
    let mut marker = marker;
    while marker != 0 {
        let slot = e.call(0x0068_15c0, &args![marker]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        let slot = e.call(0x0068_15c0, &args![marker]).u32();
        let record = e.mem.u32(slot);
        marker = e.call(0x0072_6070, &args![marker]).u32();
        let first_reference = e.mem.u32(record + 4);
        if e.call(0x0044_0da0, &args![first_reference]).bool() {
            continue;
        }
        let reference = e.mem.u32(record + 4);
        let marker_position = compass_position(e, reference);
        let distance = compass_distance_squared(e, marker_position, player_position);
        if distance > landmark_limit_squared as f32 {
            continue;
        }
        let map_data = e.mem.u32(record);
        let travel = e.call(0x0043_8ef0, &args![map_data]).bool();
        let radius_reference = e.mem.u32(record + 4);
        let radius = e.call(0x0056_8cb0, &args![radius_reference]).f32();
        let radius_squared = (f64::from(radius) * f64::from(radius)) as f32;
        let mut changed = false;
        if distance < radius_squared {
            let map_data = e.mem.u32(record);
            if !e.call(0x0043_8ed0, &args![map_data]).bool() {
                changed = true;
                let map_data = e.mem.u32(record);
                e.call(0x0044_de40, &args![map_data, 1_u32]);
                let changed_reference = e.mem.u32(record + 4);
                e.vcall(changed_reference, 0x48, &args![0x8000_0000_u32]);
            }
            if !travel {
                changed = true;
                let map_data = e.mem.u32(record);
                e.call(0x0044_de80, &args![map_data, 1_u32]);
                let changed_reference = e.mem.u32(record + 4);
                e.vcall(changed_reference, 0x48, &args![0x8000_0000_u32]);
            }
        }
        if !sneaking {
            if changed {
                let map_data = e.mem.u32(record);
                // 00779509: the five pushed words and `location` are 0076b960's;
                // 00438eb0 and 00403df0 take only `this` (plain RET).
                let location = e.call(0x0043_8eb0, &args![map_data]).u32();
                let title = e.call(0x0040_3df0, &args![0x011d_2208_u32]).u32();
                fn_0076b960(
                    e,
                    Ptr::new(title),
                    Ptr::new(location),
                    1,
                    0,
                    -1,
                    -1,
                    Ptr::new(0x0106_f370),
                );
                let notified_reference = e.mem.u32(record + 4);
                e.call(
                    0x005f_5950,
                    &args![
                        4_u32,
                        1_u32,
                        notified_reference,
                        notified_reference,
                        0_u32,
                        0_u32,
                        0_u32
                    ],
                );
                e.call(0x004d_5c60, &args![1_u32]);
                let level = blink_setting(e, 0x011d_0364);
                let player = current_player(e);
                e.vcall(player, 0x488, &args![level]);
                location_changed = 1;
            } else if travel {
                location_changed = 0;
            }
            let active = !changed && travel && distance < radius_squared;
            if !active {
                e.with_stack(8, |e, state| {
                    let reference = e.mem.u32(record + 4);
                    e.mem.set_u32(state.addr(), reference);
                    e.mem.set_u8(state.addr() + 4, 0);
                    e.mem.set_u8(state.addr() + 5, location_changed);
                    fn_00772aa0(e, Ptr::new(reference), state);
                });
            }
        }
        if landmark_tiles == 0 {
            continue;
        }
        let tile = compass_take_tile(e, landmarks, &mut landmark_tiles);
        let texture = if travel { 0x0107_35d0 } else { 0x0107_35a4 };
        compass_set_texture(e, tile, texture);
        if tile == 0 {
            continue;
        }
        let angle = hud_main_menu_get_compass_angle(
            e,
            marker_position[0],
            marker_position[1],
            marker_position[2],
            player_position[0],
            player_position[1],
            player_position[2],
        ) as f32;
        let abs_angle = e.call(0x0040_8840, &args![angle]).f64();
        let hud = current_hud(e);
        let half_angle = e.mem.i32(hud + 0x218);
        if abs_angle.partial_cmp(&f64::from(half_angle)) != Some(std::cmp::Ordering::Less) {
            hud_trait(e, tile, 0xfa3, 0);
            continue;
        }
        let hud = current_hud(e);
        let display_width = e.mem.i32(hud + 0x214).wrapping_shl(1);
        let hud = current_hud(e);
        let half_angle = e.mem.i32(hud + 0x218);
        let x = compass_interpolate(
            e,
            0.0,
            display_width as f32,
            (-half_angle) as f32,
            half_angle as f32,
            angle,
        );
        let heading_trait = e.mem.u32(0x011d_9e60);
        hud_set_float(e, tile, heading_trait, x);
        let distance_trait = e.mem.u32(0x011d_9e64);
        hud_set_float(e, tile, distance_trait, distance);
        hud_trait(e, tile, 0xfa3, 1);
        if x > inner_width {
            let fade = compass_interpolate(e, 1.0, 0.0, inner_width, outer_width, x);
            hud_set_float(
                e,
                tile,
                0xfa9,
                (f64::from(fade) * f64::from(max_alpha)) as f32,
            );
        } else if f64::from(distance) < e.global::<f64>(0x0102_f078) {
            let blink_alpha = e.mem.f32(0x011d_979c);
            blink_tile(e, Ptr::new(tile), distance, blink_alpha);
        } else {
            hud_set_float(e, tile, 0xfa9, max_alpha);
        }
    }
    while landmark_tiles != 0 {
        let tile = compass_take_tile(e, landmarks, &mut landmark_tiles);
        hud_trait(e, tile, 0xfa3, 0);
    }
    let player = current_player(e);
    let target_list = e.call(0x0095_2ba0, &args![player]).u32();
    let hud = current_hud(e);
    e.mem.set_u32(hud + 0x20c, target_list);
    let mut objective_tiles = e.call(0x0072_6070, &args![objectives.addr() + 4]).u32();
    let objective_limit = blink_setting(e, 0x011d_0bd8) as f32;
    let objective_limit_squared = (f64::from(objective_limit) * f64::from(objective_limit)) as f32;
    let hud = current_hud(e);
    let mut target_list = e.mem.u32(hud + 0x20c);
    while target_list != 0 {
        let slot = e.call(0x0068_15c0, &args![target_list]).u32();
        if e.mem.u32(slot) == 0 {
            break;
        }
        let slot = e.call(0x0068_15c0, &args![target_list]).u32();
        let target_record = e.mem.u32(slot);
        target_list = e.call(0x0072_6070, &args![target_list]).u32();
        let target = e.call(0x005c_bb70, &args![target_record]).u32();
        let actor_marker = target != 0 && e.call(0x0040_1170, &args![target]).u32() == 0x1c;
        if target == 0 {
            continue;
        }
        let enabled = e.call(0x0040_8d60, &args![0x011e_07f0_u32]).u32();
        if e.mem.u8(enabled) == 0 {
            continue;
        }
        let position = compass_position(e, target);
        let distance = compass_distance_squared(e, position, player_position);
        // 007799b9 / 00779a36: the faded alpha goes to [EBP-0xb4], which the
        // function never reads; the calls still run.
        let mut _alpha = max_alpha;
        if distance != 0.0 {
            let scaled = e
                .call(0x0040_ebd0, &args![distance, objective_limit_squared])
                .f64();
            let ratio = (scaled / f64::from(objective_limit_squared)) as f32;
            let half: f64 = e.global(0x0101_1590);
            let half_alpha = (f64::from(max_alpha) / half) as f32;
            _alpha = compass_interpolate(e, half_alpha, max_alpha, ratio, 0.0, distance);
        }
        if objective_tiles == 0 {
            continue;
        }
        let tile = compass_take_objective_tile(e, objectives, &mut objective_tiles);
        compass_set_texture(e, tile, 0x0107_3570);
        if tile == 0 {
            continue;
        }
        let mut angle = hud_main_menu_get_compass_angle(
            e,
            position[0],
            position[1],
            position[2],
            player_position[0],
            player_position[1],
            player_position[2],
        ) as f32;
        let hud = current_hud(e);
        let half_angle = e.mem.i32(hud + 0x218);
        if angle > half_angle as f32 {
            angle = half_angle as f32;
        } else if angle < (-half_angle) as f32 {
            angle = (-half_angle) as f32;
        }
        let hud = current_hud(e);
        let display_width = e.mem.i32(hud + 0x214).wrapping_shl(1);
        let hud = current_hud(e);
        let half_angle = e.mem.i32(hud + 0x218);
        let mut x = compass_interpolate(
            e,
            0.0,
            display_width as f32,
            (-half_angle) as f32,
            half_angle as f32,
            angle,
        );
        let x_cap = (f64::from(inner_width) + e.global::<f64>(0x0107_3568)) as f32;
        if x > x_cap {
            x = x_cap;
        }
        let heading_trait = e.mem.u32(0x011d_9e60);
        hud_set_float(e, tile, heading_trait, x);
        let distance_trait = e.mem.u32(0x011d_9e64);
        hud_set_float(e, tile, distance_trait, distance);
        hud_trait(e, tile, 0xfa3, 1);
        let menu = e.call(0x00a0_3c90, &args![compass]).u32();
        if e.call(0x0059_bb30, &args![menu]).u32() == 1 {
            if actor_marker {
                // 00779c4c: FLD [EBP-0x20], the max alpha, not the faded [EBP-0xb4].
                hud_set_float(e, tile, 0xfa9, max_alpha);
            } else {
                let blink_alpha = e.mem.f32(0x011d_979c);
                blink_tile(e, Ptr::new(tile), distance, blink_alpha);
            }
        }
    }
    let player = current_player(e);
    let pc_marker = fn_0077a400(e, Ptr::new(player));
    if !pc_marker.is_null() && objective_tiles != 0 {
        let tile = compass_take_objective_tile(e, objectives, &mut objective_tiles);
        compass_set_texture(e, tile, 0x0107_3534);
        if tile != 0 {
            let player = current_player(e);
            let pc_marker = fn_0077a400(e, Ptr::new(player));
            let mut position = compass_position(e, pc_marker.addr());
            position[2] = player_position[2];
            let distance = compass_distance_squared(e, position, player_position);
            let angle = hud_main_menu_get_compass_angle(
                e,
                position[0],
                position[1],
                position[2],
                player_position[0],
                player_position[1],
                player_position[2],
            ) as f32;
            let abs_angle = e.call(0x0040_8840, &args![angle]).f64();
            let hud = current_hud(e);
            let half_angle = e.mem.i32(hud + 0x218);
            if abs_angle.partial_cmp(&f64::from(half_angle)) != Some(std::cmp::Ordering::Less) {
                hud_trait(e, tile, 0xfa3, 0);
            } else {
                let hud = current_hud(e);
                let display_width = e.mem.i32(hud + 0x214).wrapping_shl(1);
                let hud = current_hud(e);
                let half_angle = e.mem.i32(hud + 0x218);
                let x = compass_interpolate(
                    e,
                    0.0,
                    display_width as f32,
                    (-half_angle) as f32,
                    half_angle as f32,
                    angle,
                );
                let heading_trait = e.mem.u32(0x011d_9e60);
                hud_set_float(e, tile, heading_trait, x);
                hud_trait(e, tile, 0xfa3, 1);
                let blink_alpha = if x <= inner_width {
                    e.mem.f32(0x011d_979c)
                } else {
                    compass_interpolate(e, max_alpha, 0.0, inner_width, outer_width, x)
                };
                blink_tile(e, Ptr::new(tile), distance, blink_alpha);
            }
        }
    }
    while objective_tiles != 0 {
        let tile = compass_take_objective_tile(e, objectives, &mut objective_tiles);
        hud_trait(e, tile, 0xfa3, 0);
    }
    e.mem.set_u8(0x011d_96c4, 0);
    let mut actor_tiles = e.call(0x0055_9450, &args![actor_ticks.addr() + 4]).u32();
    let player = current_player(e);
    let vats = e.call(0x0059_5d10, &args![player]).u32();
    let player = current_player(e);
    let process = e.call(0x008d_6f30, &args![player]).u32();
    let is_actor_mode = e.call(0x0042_5fd0, &args![process]).bool();
    let first_limit = e.call(0x0040_3e20, &args![0x011c_d7d8_u32]).u32();
    let first_limit = e.mem.f32(first_limit);
    let first_limit_squared = (f64::from(first_limit) * f64::from(first_limit)) as f32;
    let second_scale = e.call(0x0040_3e20, &args![0x011c_dcbc_u32]).u32();
    let second_scale = e.mem.f32(second_scale);
    let second_limit = blink_setting(e, 0x011c_d7d8);
    let second_limit = (f64::from(second_limit) * f64::from(second_scale)) as f32;
    let second_limit_squared = (f64::from(second_limit) * f64::from(second_limit)) as f32;
    let mut count = e.call(0x005a_e380, &args![vats]).u32();
    let actual_count = e.call(0x005a_e380, &args![vats]).u32();
    let cap = blink_setting(e, 0x011d_32d0) as u32;
    if actual_count > cap {
        e.call(0x0083_fd60, &args![vats, 0x0077_b300_u32]);
        count = blink_setting(e, 0x011d_32d0) as u32;
    }
    let mut vats = vats;
    while vats != 0 && !e.call(0x0082_56d0, &args![vats]).bool() && count != 0 {
        count -= 1;
        let slot = e.call(0x0068_15c0, &args![vats]).u32();
        let record = e.mem.u32(slot);
        if record == 0 || e.mem.u8(record + 5) != 0 {
            vats = e.call(0x0072_6070, &args![vats]).u32();
            continue;
        }
        let actor = e.mem.u32(record);
        if e.vcall(actor, 0x1d0, &args![]).u32() == 0 {
            vats = e.call(0x0072_6070, &args![vats]).u32();
            continue;
        }
        let process = e.call(0x008d_6f30, &args![actor]).u32();
        if process == 0 {
            vats = e.call(0x0072_6070, &args![vats]).u32();
            continue;
        }
        let process = e.call(0x008d_6f30, &args![actor]).u32();
        if !e.call(0x0045_0ff0, &args![process]).bool() {
            vats = e.call(0x0072_6070, &args![vats]).u32();
            continue;
        }
        let first_flag = e.vcall(actor + 0xa4, 0x08, &args![0x31_u32]).u32();
        let second_flag = if first_flag == 0 {
            e.vcall(actor + 0xa4, 0x08, &args![0x30_u32]).u32()
        } else {
            0
        };
        if first_flag != 0 || second_flag != 0 {
            let entry = e.with_stack(4, |e, output| {
                e.mem.set_f32(output.addr(), 0.0);
                let player = current_player(e);
                e.call(0x005e_58f0, &args![0x45_u32, player, output]);
                e.mem.f32(output.addr())
            });
            if entry <= 0.0 {
                vats = e.call(0x0072_6070, &args![vats]).u32();
                continue;
            }
        }
        let actor = e.mem.u32(record);
        let position = compass_position(e, actor);
        let distance = compass_distance_squared(e, position, player_position);
        let limit = if is_actor_mode {
            first_limit_squared
        } else {
            second_limit_squared
        };
        if distance > limit
            || distance.partial_cmp(&(landmark_limit_squared as f32))
                != Some(std::cmp::Ordering::Less)
        {
            vats = e.call(0x0072_6070, &args![vats]).u32();
            continue;
        }
        if actor_tiles == 0 {
            continue;
        }
        let tile = compass_take_tile(e, actor_ticks, &mut actor_tiles);
        compass_set_texture(e, tile, 0x0107_350c);
        if tile != 0 {
            let angle = hud_main_menu_get_compass_angle(
                e,
                position[0],
                position[1],
                position[2],
                player_position[0],
                player_position[1],
                player_position[2],
            ) as f32;
            let abs_angle = e.call(0x0040_8840, &args![angle]).f64();
            let hud = current_hud(e);
            let half_angle = e.mem.i32(hud + 0x218);
            if abs_angle.partial_cmp(&f64::from(half_angle)) != Some(std::cmp::Ordering::Less) {
                hud_trait(e, tile, 0xfa3, 0);
            } else {
                let hud = current_hud(e);
                let display_width = e.mem.i32(hud + 0x214).wrapping_shl(1);
                let hud = current_hud(e);
                let half_angle = e.mem.i32(hud + 0x218);
                let x = compass_interpolate(
                    e,
                    0.0,
                    display_width as f32,
                    (-half_angle) as f32,
                    half_angle as f32,
                    angle,
                );
                let added = (f64::from(x) + e.global::<f64>(0x0101_1590)) as f32;
                let heading_trait = e.mem.u32(0x011d_9e60);
                hud_set_float(e, tile, heading_trait, added);
                hud_trait(e, tile, 0xfa3, 1);
                if e.mem.u8(record + 4) != 0 {
                    hud_trait(e, tile, 0xff4, 2);
                    e.mem.set_u8(0x011d_96c4, 1);
                } else {
                    hud_trait(e, tile, 0xff4, 1);
                }
                let alpha = if x <= inner_width {
                    max_alpha
                } else {
                    compass_interpolate(e, max_alpha, 0.0, inner_width, outer_width, x)
                };
                hud_set_float(e, tile, 0xfa9, alpha);
            }
        }
        vats = e.call(0x0072_6070, &args![vats]).u32();
    }
    while actor_tiles != 0 {
        let tile = compass_take_tile(e, actor_ticks, &mut actor_tiles);
        hud_trait(e, tile, 0xfa3, 0);
    }
}

// Translated from 0077a400 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the first entry of an object's list at +0x708, or its fallback
/// field at +0x6f4 when that list is empty.
pub fn fn_0077a400(e: &mut Engine, object: Ptr) -> Ptr {
    let list = object.addr() + 0x708;
    if e.call(0x0044_ddc0, &args![list]).u32() != 0 {
        let entry = e.call(0x0055_8dd0, &args![list, 0_u32]).u32();
        Ptr::new(e.mem.u32(entry))
    } else {
        Ptr::new(e.mem.u32(object.addr() + 0x6f4))
    }
}

// Translated from 0077a450 (decompiled, FalloutNV.exe 1.4.0.525)
/// Stores a pair of float coordinates at +0x7c and +0x80.
pub fn fn_0077a450(e: &mut Engine, object: Ptr, x: f32, y: f32) {
    e.mem.set_f32(object.addr() + 0x7c, x87_quiet(x));
    e.mem.set_f32(object.addr() + 0x80, x87_quiet(y));
}

// Translated from 0077a480 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::ShowQuestName` (Xbox PDB): queues a quest name update and
/// adjusts the player's active quest based on its completion/failure flags.
pub fn hud_main_menu_show_quest_name(e: &mut Engine, quest: Ptr) {
    e.call(0x0045_38a0, &args![0x011d_96cc_u32, 0_u32]);
    if !quest.is_null() && current_hud(e) != 0 {
        let now = e.call(0x0082_5c00, &args![0x011f_6394_u32]).u32();
        e.mem.set_u32(0x011d_96ac, now);
        let entry = e.call(0x0040_1000, &args![0x324_u32]).u32();
        e.mem.set_u32(entry, quest.addr());
        let failed = fn_0077a590(e, quest);
        let completed = if failed {
            false
        } else {
            e.call(0x0059_e400, &args![quest]).bool()
        };
        let state = if failed {
            2
        } else if completed {
            1
        } else {
            0
        };
        e.mem.set_u32(entry + 4, state);
        let player = current_player(e);
        let active = e.call(0x005c_bb50, &args![player]).u32();
        if (failed || completed) && active == quest.addr() {
            let player = current_player(e);
            e.call(0x0095_29d0, &args![player, 0_u32]);
        } else if !failed && !completed && active == 0 {
            let player = current_player(e);
            e.call(0x0095_29d0, &args![player, quest]);
        }
        e.call(0x0076_bad0, &args![entry]);
        e.call(0x0045_38c0, &args![0x011d_96cc_u32]);
    }
}

// Translated from 0077a590 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests bit 0x40 of the quest's flag byte returned by `005a8080`.
pub fn fn_0077a590(e: &mut Engine, quest: Ptr) -> bool {
    let flags = e.call(0x005a_8080, &args![quest]).u32();
    e.mem.u8(flags) & 0x40 != 0
}

// Translated from 0077a5b0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::ShowQuestObjective` (Xbox PDB): copies at most 0x104 text
/// bytes and the two status bytes into a queued objective record.
pub fn hud_main_menu_show_quest_objective(e: &mut Engine, text: Ptr, first: u8, second: u8) {
    let hud = current_hud(e);
    if hud == 0 || text.is_null() {
        return;
    }
    e.call(0x0045_38a0, &args![0x011d_9730_u32, 0_u32]);
    let now = e.call(0x0082_5c00, &args![0x011f_6394_u32]).u32();
    e.mem.set_u32(0x011d_96ac, now);
    let entry = e.call(0x0040_1000, &args![0x106_u32]).u32();
    e.call(0x0040_6d30, &args![entry, 0x104_u32, text]);
    e.mem.set_u8(entry + 0x104, first);
    e.mem.set_u8(entry + 0x105, second);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), entry);
        let hud = current_hud(e);
        e.call(0x005a_e3d0, &args![hud + 0x258, slot]);
    });
    e.call(0x0045_38c0, &args![0x011d_9730_u32]);
}

// Translated from 0077a650 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::UpdateQuestText` (Xbox PDB): advances queued quest notices
/// only after the HUD mode and debounce checks permit it.
// Translation incomplete: 0077a650 - compiler-generated SEH unwinding is omitted.
pub fn hud_main_menu_update_quest_text(e: &mut Engine, this: Ptr<HUDMainMenu>) {
    if current_hud(e) == 0 || matches!(e.mem.i32(this.addr() + 0x1c4), 7..=9) {
        return;
    }
    let now = e.call(0x0082_5c00, &args![0x011f_6394_u32]).u32();
    if now.wrapping_sub(e.mem.u32(0x011d_96ac)) < 2000 {
        return;
    }
    e.call(0x0071_78a0, &args![]);
    let queue = Ptr::new(e.mem.u32(0x011d_96bc));
    if !hud_main_menu_is_quest_name_showing(e, this) && !fn_0077b2c0(e, queue) {
        let objectives_showing = hud_main_menu_are_objectives_showing(e, this);
        let objectives_alpha = if objectives_showing {
            let hud = current_hud(e);
            let tile_slot = e.call(0x0073_ac90, &args![e.mem.u32(hud + 0x5c) + 4]).u32();
            e.call(0x00a0_11b0, &args![e.mem.u32(tile_slot), 0xfa9_u32])
                .f64()
        } else {
            0.0
        };
        if !objectives_showing || objectives_alpha == 0.0 {
            let interface = e.call(0x004b_7210, &args![]).u32();
            let normal_state = e.call(0x0070_23a0, &args![interface]).bool();
            let unobstructed = !normal_state
                || e.call(0x0070_2680, &args![0x3f1_u32, 0_u32]).bool()
                || e.call(0x0070_3d50, &args![]).bool();
            if unobstructed && e.call(0x0044_ddc0, &args![0x011f_2250_u32]).u32() == 0 {
                e.mem.set_u8(this.addr() + 0x260, 1);
                quest_popup_start(e, this);
            }
        }
    }
    if hud_main_menu_is_quest_name_showing(e, this) {
        let hud = current_hud(e);
        e.call(0x0077_c170, &args![this, e.mem.u32(hud + 0x60)]);
    } else if !e.call(0x0070_2450, &args![0x418_u32]).bool()
        && (hud_main_menu_are_objectives_showing(e, this)
            || !e.call(0x0082_56d0, &args![this.addr() + 0x258]).bool())
        && !hud_main_menu_is_quest_name_showing(e, this)
    {
        let hud = current_hud(e);
        let reminder_tile = e.mem.u32(hud + 0x5c);
        hud_main_menu_handle_reminder_fade(e, this, Ptr::new(reminder_tile));
    }
}

fn quest_queue_head(e: &mut Engine) -> u32 {
    let queue = Ptr::new(e.mem.u32(0x011d_96bc));
    fn_0077b2a0(e, queue).addr()
}

fn quest_popup_start(e: &mut Engine, this: Ptr<HUDMainMenu>) {
    e.with_stack(24, |e, handle| {
        e.call(0x0041_a250, &args![handle]);
        let head = quest_queue_head(e);
        let kind = e.mem.u32(head + 4);
        let sound_name = match kind {
            0 => 0x0106_f370,
            3 => quest_queue_head(e) + 0x220,
            _ => 0x0107_3700,
        };
        e.with_stack(12, |e, temporary| {
            let audio = e.call(0x0045_3a70, &args![]).u32();
            let sound = e
                .call(0x00ad_7550, &args![audio, temporary, sound_name, 0x121_u32])
                .u32();
            e.call(0x0041_8900, &args![handle, sound]);
            e.call(0x0048_3710, &args![temporary]);
        });
        e.call(0x00ad_8830, &args![handle, 0_u32]);
        let now = e.call(0x0082_5c00, &args![0x011f_6394_u32]).u32();
        let hud = current_hud(e);
        let tile_list = e.mem.u32(hud + 0x60);
        let tile_slot = e.call(0x0073_ac90, &args![tile_list + 4]).u32();
        let tile = e.mem.u32(tile_slot);
        let head = quest_queue_head(e);
        let kind = e.mem.u32(head + 4);
        let text = match kind {
            0..=2 => {
                let setting = match kind {
                    0 => 0x011d_282c,
                    2 => 0x011d_27c0,
                    _ => 0x011d_237c,
                };
                e.call(0x0040_3df0, &args![setting]).u32()
            }
            3 => quest_queue_head(e) + 8,
            _ => {
                e.call(0x005b_5e40, &args![7_u32, 0x0107_36b0_u32, kind]);
                0
            }
        };
        if kind <= 3 {
            e.call(0x00a0_1350, &args![tile, 0xfc4_u32, text, 1_u32]);
        }
        hud_trait(e, tile, 0xfa3, 1);
        hud_trait(e, tile, 0xfa9, 0);
        let mut font = 7;
        let head = quest_queue_head(e);
        if e.mem.u32(head + 4) == 3 {
            let head = quest_queue_head(e);
            if e.mem.i32(head + 0x218) != -1 {
                let head = quest_queue_head(e);
                font = e.mem.u32(head + 0x218);
            }
        }
        hud_trait(e, tile, 0xfb9, font);
        let (available_width, justification_factor, text_width) =
            e.with_stack(12, |e, dimensions| {
                let scale = e.call(0x00a0_11b0, &args![tile, 0xfba_u32]).f32();
                let tile_font = e.call(0x00a0_11b0, &args![tile, 0xfb9_u32]).f64();
                let tile_font = e.call(0x00ec_62c0, &args![tile_font]).i32();
                let text = e.call(0x00a0_11f0, &args![tile, 0xfc4_u32]).u32();
                let manager = e.call(0x005b_d5b0, &args![]).u32();
                e.call(
                    0x00a1_b020,
                    &args![manager, dimensions, text, tile_font, scale, 0_u32],
                );
                let measured_width = e.mem.f32(dimensions.addr());
                let text_width = e.call(0x00ec_62c0, &args![f64::from(measured_width)]).i32();
                let top = -(f64::from(e.mem.f32(dimensions.addr() + 4))
                    + e.global::<f64>(0x0101_5a38)) as f32;
                hud_set_float(e, tile, 0xfa2, top);
                let width = e.call(0x0071_5d40, &args![]).f64();
                let width = e.call(0x00ec_62c0, &args![width]).i32();
                let margin = e.call(0x0071_77c0, &args![]).f64();
                let margin = e.call(0x00ec_62c0, &args![margin]).i32();
                let available_width = width.wrapping_sub(margin.wrapping_mul(2));
                let mut justification_factor = 0.0_f32;
                let head = quest_queue_head(e);
                if e.mem.u32(head + 4) == 3 {
                    let head = quest_queue_head(e);
                    let justification = e.mem.i32(head + 0x210);
                    let (alignment, factor) = match justification {
                        0 => (1, 0.0_f32),
                        1 => (2, e.global::<f32>(0x0101_6248)),
                        2 => (4, 1.0_f32),
                        _ => {
                            e.call(
                                0x005b_5e40,
                                &args![7_u32, 0x0107_3640_u32, justification, 0_u32],
                            );
                            (1, 0.0_f32)
                        }
                    };
                    let half_width = text_width / 2;
                    let x = (f64::from(available_width.wrapping_sub(half_width))
                        * f64::from(factor)) as f32;
                    hud_set_float(e, tile, 0xfa1, x);
                    hud_trait(e, tile, 0xfb7, alignment);
                    justification_factor = factor;
                }
                (available_width, justification_factor, text_width)
            });
        // 0077ac19: the show time goes through 00715c60 (FILD qword of the
        // zero-extended tick, so unsigned), not the signed 00700320.
        e.call(0x0071_5c60, &args![tile, 0x1004_u32, now]);
        let timing_setting = e.call(0x0040_3e20, &args![0x011d_4d14_u32]).u32();
        let duration = (f64::from(e.mem.f32(timing_setting)) * e.global::<f64>(0x0101_7b70)
            + f64::from(now)) as f32;
        hud_set_float(e, tile, 0x1005, duration);
        hud_main_menu_set_brightness(e, Ptr::new(tile), 0);
        quest_popup_text(
            e,
            this,
            available_width,
            justification_factor,
            text_width,
            now,
        );
        e.call(0x0048_3710, &args![handle]);
    });
}

fn quest_popup_text(
    e: &mut Engine,
    this: Ptr<HUDMainMenu>,
    available_width: i32,
    justification_factor: f32,
    mut text_width: i32,
    mut now: u32,
) {
    let head = quest_queue_head(e);
    if e.mem.u32(head + 4) == 1 {
        hud_main_menu_kill_quest_objectives(e, this);
    }
    e.with_stack(0x104, |e, title| {
        let head = quest_queue_head(e);
        let source = if e.mem.u32(head + 4) == 3 {
            quest_queue_head(e) + 0x10c
        } else {
            let record = quest_queue_head(e);
            let location = e.mem.u32(record);
            e.call(0x0040_8da0, &args![location + 0x30]).u32()
        };
        e.call(0x0040_6d30, &args![title, 0x104_u32, source]);
        for index in 0..0x104_u32 {
            let letter = i32::from(e.mem.u8(title.addr() + index) as i8);
            let upper = e.call(0x00ec_a7f4, &args![letter]).u32();
            e.mem.set_u8(title.addr() + index, upper as u8);
        }
        let length = e.call(0x0044_a670, &args![title]).i32();
        let mut wrap = 32_i32;
        if length >= 32 {
            for index in (0..=32_u32).rev() {
                if e.mem.u8(title.addr() + index) == b' ' {
                    wrap = index as i32;
                    break;
                }
            }
        }
        let mut font = 8;
        let head = quest_queue_head(e);
        if e.mem.u32(head + 4) == 3 {
            let head = quest_queue_head(e);
            if e.mem.i32(head + 0x21c) != -1 {
                let head = quest_queue_head(e);
                font = e.mem.u32(head + 0x21c);
            }
        }
        let max_width: f32 = e.global(0x0101_6970);
        let glyph_width = e.with_stack(12, |e, dimensions| {
            let manager = e.call(0x005b_d5b0, &args![]).u32();
            e.call(
                0x00a1_b020,
                &args![manager, dimensions, title, font, max_width, 0_u32],
            );
            e.mem.f32(dimensions.addr())
        });
        let target_x = (f64::from(available_width)
            - (f64::from(justification_factor) + 1.0) * f64::from(glyph_width))
            * f64::from(justification_factor);
        let mut left = e.call(0x00ec_62c0, &args![target_x]).i32();
        let hud = current_hud(e);
        let pool = e.mem.u32(hud + 0x60);
        let mut tiles = e.call(0x0055_9450, &args![pool + 4]).u32();
        if tiles != 0 {
            compass_take_tile(e, Ptr::new(pool), &mut tiles);
        }
        let mut index = 0_u32;
        loop {
            let letter = e.mem.u8(title.addr() + index);
            if letter == 0 {
                break;
            }
            let current_index = index;
            index = index.wrapping_add(1);
            if tiles == 0 {
                continue;
            }
            let hud = current_hud(e);
            let pool = e.mem.u32(hud + 0x60);
            let glyph_tile = compass_take_tile(e, Ptr::new(pool), &mut tiles);
            e.with_stack(2, |e, character| {
                e.mem.set_u8(character.addr(), letter);
                e.mem.set_u8(character.addr() + 1, 0);
                if current_index as i32 == wrap + 1 {
                    left = e.call(0x00ec_62c0, &args![target_x]).i32();
                }
                hud_trait(e, glyph_tile, 0xfb9, font);
                e.call(0x00a0_1350, &args![glyph_tile, 0xfc4_u32, character, 1_u32]);
                hud_trait(e, glyph_tile, 0xfa3, 1);
                hud_trait(e, glyph_tile, 0xfa9, 0);
                hud_trait(e, glyph_tile, 0xfa1, left as u32);
                let scale = e.call(0x00a0_11b0, &args![glyph_tile, 0xfba_u32]).f32();
                let tile_font = e.call(0x00a0_11b0, &args![glyph_tile, 0xfb9_u32]).f64();
                let tile_font = e.call(0x00ec_62c0, &args![tile_font]).i32();
                let glyph_width = e.with_stack(12, |e, dimensions| {
                    let manager = e.call(0x005b_d5b0, &args![]).u32();
                    e.call(
                        0x00a1_b020,
                        &args![manager, dimensions, character, tile_font, scale, 0_u32],
                    );
                    let width = e.mem.f32(dimensions.addr());
                    if current_index as i32 > wrap {
                        let y = (f64::from(e.mem.f32(dimensions.addr() + 4))
                            + e.global::<f64>(0x0102_0998)) as f32;
                        hud_set_float(e, glyph_tile, 0xfa2, y);
                    } else {
                        hud_trait(e, glyph_tile, 0xfa2, 0);
                    }
                    width
                });
                let pause_setting = e.call(0x0040_3e20, &args![0x011d_2688_u32]).u32();
                let pause = f64::from(e.mem.f32(pause_setting)) * e.global::<f64>(0x0101_7b70);
                now = now.wrapping_add(fistp_i64_low(pause));
                e.call(0x0071_5c60, &args![glyph_tile, 0x1004_u32, now]);
                let duration_setting = e.call(0x0040_3e20, &args![0x011d_4d14_u32]).u32();
                let end = (f64::from(e.mem.f32(duration_setting)) * e.global::<f64>(0x0101_7b70)
                    + f64::from(now)) as f32;
                hud_set_float(e, glyph_tile, 0x1005, end);
                let duration_setting = e.call(0x0040_3e20, &args![0x011d_4d14_u32]).u32();
                let reported_end = f64::from(e.mem.f32(duration_setting))
                    * e.global::<f64>(0x0101_7b70)
                    + f64::from(now);
                let text = e.call(0x00a0_11f0, &args![glyph_tile, 0xfc4_u32]).u32();
                e.call(0x0040_fbe0, &args![0x0107_3624_u32, text, reported_end]);
                hud_main_menu_set_brightness(e, Ptr::new(glyph_tile), 0);
                let width = e.call(0x00ec_62c0, &args![f64::from(glyph_width)]).i32();
                left = left.wrapping_add(width);
                let x = e.call(0x00a0_11b0, &args![glyph_tile, 0xfa1_u32]).f64();
                let right = x + f64::from(glyph_width);
                if f64::from(text_width) < right {
                    text_width = e.call(0x00ec_62c0, &args![right]).i32();
                }
            });
        }
        let head = quest_queue_head(e);
        e.call(0x0040_1030, &args![head]);
        let queue = Ptr::new(e.mem.u32(0x011d_96bc));
        fn_0077b2e0(e, queue);
        let hud = current_hud(e);
        let tile_list = e.mem.u32(hud + 0x60);
        e.call(0x0077_c170, &args![this, tile_list]);
    });
}

// Translated from 0077b2a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Returns the first pointer stored by the fixed quest queue at 011d970c.
pub fn fn_0077b2a0(e: &mut Engine, _unused_this: Ptr) -> Ptr {
    let slot = e.call(0x0068_15c0, &args![0x011d_970c_u32]).u32();
    Ptr::new(e.mem.u32(slot))
}

// Translated from 0077b2c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Tests whether the fixed quest queue at 011d970c is empty.
pub fn fn_0077b2c0(e: &mut Engine, _unused_this: Ptr) -> bool {
    e.call(0x0082_56d0, &args![0x011d_970c_u32]).bool()
}

// Translated from 0077b2e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Removes the head of the fixed quest queue at 011d970c.
pub fn fn_0077b2e0(e: &mut Engine, _unused_this: Ptr) {
    e.call(0x0063_f7b0, &args![0x011d_970c_u32]);
}

// Translated from 0077b300 (decompiled, FalloutNV.exe 1.4.0.525)
/// Compares two actor pointers stored in slots by squared distance to the player.
/// Null slots compare equally; ties and unordered float distances also return zero.
pub fn fn_0077b300(e: &mut Engine, first_slot: Ptr, second_slot: Ptr) -> i32 {
    if first_slot.is_null()
        || e.mem.u32(first_slot.addr()) == 0
        || second_slot.is_null()
        || e.mem.u32(second_slot.addr()) == 0
    {
        return 0;
    }
    let player = current_player(e);
    let player_position = compass_position(e, player);
    let first_actor = e.mem.u32(first_slot.addr());
    let first_position = compass_position(e, first_actor);
    let first_distance = compass_distance_squared(e, first_position, player_position);
    let second_actor = e.mem.u32(second_slot.addr());
    let second_position = compass_position(e, second_actor);
    let second_distance = compass_distance_squared(e, second_position, player_position);
    if second_distance < first_distance {
        1
    } else if first_distance < second_distance {
        -1
    } else {
        0
    }
}

fn reminder_advance(e: &mut Engine, pool: u32, cursor: &mut u32, address: u32) -> u32 {
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), *cursor);
        let element = e.call(address, &args![pool + 4, slot]).u32();
        *cursor = e.mem.u32(slot.addr());
        element
    })
}

fn reminder_fade(e: &mut Engine, tile: u32, trait_id: u32, start: f32, end: f32, seconds: f32) {
    e.call(
        0x00a0_7c60,
        &args![tile, trait_id, start, end, seconds, 0_u32],
    );
}

fn reminder_setting(e: &mut Engine, address: u32) -> f32 {
    let setting = e.call(0x0040_3e20, &args![address]).u32();
    e.mem.f32(setting)
}

fn reminder_global_trait_value(e: &mut Engine, tile: u32, trait_address: u32) -> f64 {
    let trait_id = e.mem.u32(trait_address);
    e.call(0x00a0_11b0, &args![tile, trait_id]).f64()
}

// Translated from 0077b430 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::HandleReminderFade` (Xbox PDB): places queued objective
/// reminders into the HUD tile pool and advances their fade controls.
// Translation incomplete: 0077b430 - compiler SEH unwinding.
pub fn hud_main_menu_handle_reminder_fade(e: &mut Engine, this: Ptr<HUDMainMenu>, tile: Ptr) {
    let screen_width = e.call(0x0071_5d40, &args![]).f64();
    let screen_width = x87_to_int(e, screen_width);
    let mut placed = false;
    if !hud_main_menu_are_objectives_showing(e, this) {
        let mut queue = this.addr() + 0x258;
        let hud = current_hud(e);
        let pool = e.mem.u32(hud + 0x5c);
        let mut marker_tiles = e.call(0x0055_9450, &args![pool + 4]).u32();
        for _ in 0..31 {
            let hud = current_hud(e);
            let pool = e.mem.u32(hud + 0x5c);
            reminder_advance(e, pool, &mut marker_tiles, 0x0057_cbe0);
        }
        let hud = current_hud(e);
        let pool = e.mem.u32(hud + 0x5c);
        let mut icon_tiles = e.call(0x0072_6070, &args![pool + 4]).u32();
        while queue != 0 && !e.call(0x0082_56d0, &args![queue]).bool() && marker_tiles != 0 {
            let objectives_showing = hud_main_menu_are_objectives_showing(e, this);
            let reminder_visible = if objectives_showing {
                let trait_id = e.call(0x00a0_1860, &args![0x0107_37b4_u32]).u32();
                let hud = current_hud(e);
                let pool = e.mem.u32(hud + 0x5c);
                let slot = e.call(0x0073_ac90, &args![pool + 4]).u32();
                let tile = e.mem.u32(slot);
                e.call(0x00a0_1230, &args![tile, trait_id]).bool()
            } else {
                false
            };
            let slot = e.call(0x0068_15c0, &args![queue]).u32();
            let record = e.mem.u32(slot);
            let is_reminder = e.mem.u8(record + 0x105) != 0;
            if objectives_showing && reminder_visible != is_reminder {
                queue = e.call(0x0072_6070, &args![queue]).u32();
                continue;
            }
            e.mem.set_u8(this.addr() + 0x261, 1);
            placed = true;
            let hud = current_hud(e);
            let pool = e.mem.u32(hud + 0x5c);
            let marker_slot = reminder_advance(e, pool, &mut marker_tiles, 0x0068_3520);
            let marker_tile = e.mem.u32(marker_slot);
            let hud = current_hud(e);
            let pool = e.mem.u32(hud + 0x5c);
            let icon_slot = reminder_advance(e, pool, &mut icon_tiles, 0x0068_3520);
            let icon_tile = e.mem.u32(icon_slot);
            let slot = e.call(0x0068_15c0, &args![queue]).u32();
            let record = e.mem.u32(slot);
            if e.mem.u8(record + 0x104) == 0 {
                let slot = e.call(0x0068_15c0, &args![queue]).u32();
                let text = e.mem.u32(slot);
                e.call(0x00a0_1350, &args![marker_tile, 0xfc4_u32, text, 1_u32]);
                hud_trait(e, marker_tile, 0x1007, 0);
                let trait_id = e.call(0x00a0_1860, &args![0x0107_37b4_u32]).u32();
                hud_trait(e, marker_tile, trait_id, u32::from(is_reminder));
                e.call(
                    0x00a0_1350,
                    &args![icon_tile, 0xfcc_u32, 0x0107_374c_u32, 1_u32],
                );
            } else {
                e.with_stack(0x104, |e, buffer| {
                    let prefix = e.call(0x0040_3df0, &args![0x011d_1d60_u32]).u32();
                    e.call(0x0040_6d30, &args![buffer, 0x104_u32, prefix]);
                    let slot = e.call(0x0068_15c0, &args![queue]).u32();
                    let text = e.mem.u32(slot);
                    e.call(0x0040_6d50, &args![buffer, 0x104_u32, text]);
                    e.call(0x00a0_1350, &args![marker_tile, 0xfc4_u32, buffer, 1_u32]);
                });
                let trait_id = e.call(0x00a0_1860, &args![0x0107_37b4_u32]).u32();
                hud_trait(e, marker_tile, trait_id, u32::from(is_reminder));
                hud_trait(e, marker_tile, 0x1007, 1);
                e.call(
                    0x00a0_1350,
                    &args![icon_tile, 0xfcc_u32, 0x0107_377c_u32, 1_u32],
                );
            }
            let icon_x = e.call(0x00a0_11b0, &args![marker_tile, 0xfa1_u32]).f64();
            let icon_x = (icon_x - e.global::<f64>(0x0102_fc70)) as f32;
            hud_set_float(e, icon_tile, 0xfa1, icon_x);
            hud_trait(e, icon_tile, 0xfa3, 1);
            hud_trait(e, marker_tile, 0xfb9, 7);
            hud_trait(e, marker_tile, 0xfa3, 1);
            hud_trait(e, marker_tile, 0xfa9, 0);
            hud_trait(e, marker_tile, 0xfba, (screen_width / 3) as u32);
            e.with_stack(0x104, |e, name| {
                e.call(0x0040_6d00, &args![name, 0x104_u32, 0x0107_3744_u32, 0_u32]);
                e.call(0x0070_7e10, &args![marker_tile, name]);
            });
            hud_main_menu_set_brightness(e, Ptr::new(marker_tile), 0);
            e.with_stack(12, |e, temporary| {
                let audio = e.call(0x0045_3a70, &args![]).u32();
                let handle = e
                    .call(
                        0x00ad_7550,
                        &args![audio, temporary, 0x0107_3734_u32, 0x121_u32],
                    )
                    .u32();
                e.call(0x00ad_8830, &args![handle, 0_u32]);
                e.call(0x0048_3710, &args![temporary]);
            });
            let slot = e.call(0x0068_15c0, &args![queue]).u32();
            e.call(0x0090_5330, &args![queue, slot]);
        }
    }
    if placed {
        let hud = current_hud(e);
        let pool = e.mem.u32(hud + 0x5c);
        let mut marker_tiles = e.call(0x0055_9450, &args![pool + 4]).u32();
        let hud = current_hud(e);
        let pool = e.mem.u32(hud + 0x5c);
        let mut icon_tiles = e.call(0x0072_6070, &args![pool + 4]).u32();
        for _ in 0..31 {
            let hud = current_hud(e);
            let pool = e.mem.u32(hud + 0x5c);
            reminder_advance(e, pool, &mut icon_tiles, 0x0068_3520);
        }
        let mut cumulative_width = 0_i32;
        for _ in 0..32 {
            if marker_tiles == 0 {
                break;
            }
            let marker_slot = reminder_advance(e, tile.addr(), &mut marker_tiles, 0x0057_cbe0);
            let marker_tile = e.mem.u32(marker_slot);
            let icon_slot = reminder_advance(e, tile.addr(), &mut icon_tiles, 0x0057_cbe0);
            let icon_tile = e.mem.u32(icon_slot);
            if marker_tile == 0 || !e.call(0x00a0_1230, &args![marker_tile, 0xfa3_u32]).bool() {
                continue;
            }
            hud_trait(e, marker_tile, 0xfa2, cumulative_width as u32);
            hud_trait(e, icon_tile, 0xfa2, cumulative_width.wrapping_add(3) as u32);
            let scale = e.call(0x00a0_11b0, &args![marker_tile, 0xfba_u32]).f32();
            let font = e.call(0x00a0_11b0, &args![marker_tile, 0xfb9_u32]).f64();
            let font = x87_to_int(e, font);
            let text = e.call(0x00a0_11f0, &args![marker_tile, 0xfc4_u32]).u32();
            let width = e.with_stack(12, |e, dimensions| {
                let manager = e.call(0x005b_d5b0, &args![]).u32();
                e.call(
                    0x00a1_b020,
                    &args![manager, dimensions, text, font, scale, 0_u32],
                );
                e.mem.f32(dimensions.addr() + 4)
            });
            cumulative_width = cumulative_width.wrapping_add(x87_to_int(e, f64::from(width)));
        }
    }
    if e.mem.u32(0x011d_9e74) & 1 == 0 {
        let flags = e.mem.u32(0x011d_9e74);
        e.mem.set_u32(0x011d_9e74, flags | 1);
        let wait_complete = e.call(0x00a0_0940, &args![0x0107_3724_u32, -1_i32]).u32();
        e.mem.set_u32(0x011d_9e70, wait_complete);
    }
    let mut marker_tiles = e.call(0x0055_9450, &args![tile.addr() + 4]).u32();
    let mut icon_tiles = e.call(0x0072_6070, &args![tile.addr() + 4]).u32();
    for _ in 0..31 {
        let hud = current_hud(e);
        let pool = e.mem.u32(hud + 0x5c);
        reminder_advance(e, pool, &mut icon_tiles, 0x0068_3520);
    }
    let mut markers = [0_u32; 4];
    let mut icons = [0_u32; 4];
    let mut count = 0_usize;
    for _ in 0..32 {
        if marker_tiles == 0 || count >= 4 {
            break;
        }
        let marker_slot = reminder_advance(e, tile.addr(), &mut marker_tiles, 0x0057_cbe0);
        let marker = e.mem.u32(marker_slot);
        let icon_slot = reminder_advance(e, tile.addr(), &mut icon_tiles, 0x0057_cbe0);
        let icon = e.mem.u32(icon_slot);
        if !e.call(0x00a0_1230, &args![marker, 0xfa3_u32]).bool() {
            continue;
        }
        if count > 0 && markers[count] == icons[count] {
            break;
        }
        markers[count] = marker;
        icons[count] = icon;
        count += 1;
    }
    if count == 0 {
        return;
    }
    let wait_trait = e.mem.u32(0x011d_9e70);
    let wait = e.call(0x00a0_11b0, &args![markers[0], wait_trait]).f64();
    let zero = e.global::<f64>(0x0101_2060);
    let one = e.global::<f64>(0x0101_2070);
    if wait == zero {
        let wait_trait = e.mem.u32(0x011d_9e70);
        hud_trait(e, markers[0], wait_trait, 0);
        let half_second = e.global::<f32>(0x0101_6248);
        let alpha = e.global::<f32>(0x011d_979c);
        reminder_fade(e, markers[0], 0xfa9, 0.0, alpha, half_second);
        let seconds =
            (f64::from(reminder_setting(e, 0x011d_4cc0)) * e.global::<f64>(0x0102_1928)) as f32;
        let wait_trait = e.mem.u32(0x011d_9e70);
        reminder_fade(e, markers[0], wait_trait, 0.0, 1.0, seconds);
        let alpha = e.global::<f32>(0x011d_979c);
        let half_second = e.global::<f32>(0x0101_6248);
        reminder_fade(e, icons[0], 0xfa9, 0.0, alpha, half_second);
        if markers[1] != 0 {
            let seconds = reminder_setting(e, 0x011d_4cc0);
            let wait_trait = e.mem.u32(0x011d_9e70);
            reminder_fade(e, markers[1], wait_trait, 0.0, 1.0, seconds);
            if markers[2] != 0 {
                let seconds = (f64::from(reminder_setting(e, 0x011d_4cc0))
                    * e.global::<f64>(0x0101_1590)) as f32;
                let wait_trait = e.mem.u32(0x011d_9e70);
                reminder_fade(e, markers[2], wait_trait, 0.0, 1.0, seconds);
            }
        }
    } else if reminder_global_trait_value(e, markers[0], 0x011d_9e70) == one {
        if e.mem.u32(0x011d_9e74) & 2 == 0 {
            let flags = e.mem.u32(0x011d_9e74);
            e.mem.set_u32(0x011d_9e74, flags | 2);
            let hold = e.call(0x00a0_0940, &args![0x0107_3718_u32, -1_i32]).u32();
            e.mem.set_u32(0x011d_9e6c, hold);
        }
        let hold_trait = e.mem.u32(0x011d_9e6c);
        let hold = e.call(0x00a0_11b0, &args![markers[0], hold_trait]).f64();
        if hold == zero {
            let hold_trait = e.mem.u32(0x011d_9e6c);
            hud_trait(e, markers[0], hold_trait, 0);
            let seconds = reminder_setting(e, 0x011d_3b68);
            let hold_trait = e.mem.u32(0x011d_9e6c);
            reminder_fade(e, markers[0], hold_trait, 0.0, 1.0, seconds);
        } else if reminder_global_trait_value(e, markers[0], 0x011d_9e6c) == one {
            let seconds = reminder_setting(e, 0x011d_2b5c);
            let alpha = e.global::<f32>(0x011d_979c);
            reminder_fade(e, markers[0], 0xfa9, alpha, 0.0, seconds);
            let wait_trait = e.mem.u32(0x011d_9e70);
            hud_trait(e, markers[0], wait_trait, 2);
            let seconds = reminder_setting(e, 0x011d_2b5c);
            let alpha = e.global::<f32>(0x011d_979c);
            reminder_fade(e, icons[0], 0xfa9, alpha, 0.0, seconds);
        }
    } else if e.call(0x00a0_7fc0, &args![markers[0], 0xfa9_u32]).bool()
        && e.call(0x00a0_11b0, &args![markers[0], 0xfa9_u32]).f64() == zero
    {
        if markers[1] != 0 {
            let second_x = e.call(0x00a0_11b0, &args![markers[1], 0xfa2_u32]).f64();
            let first_x = e.call(0x00a0_11b0, &args![markers[0], 0xfa2_u32]).f64();
            let shift = x87_to_int(e, second_x - first_x);
            let mut cursor = e.call(0x0055_9450, &args![tile.addr() + 4]).u32();
            if cursor != 0 {
                reminder_advance(e, tile.addr(), &mut cursor, 0x0057_cbe0);
            }
            while cursor != 0 {
                let slot = reminder_advance(e, tile.addr(), &mut cursor, 0x0057_cbe0);
                let target = e.mem.u32(slot);
                let seconds = reminder_setting(e, 0x011d_2724);
                let end_x = e.call(0x00a0_11b0, &args![target, 0xfa2_u32]).f64();
                let end_x = (end_x - f64::from(shift)) as f32;
                let start_x = e.call(0x00a0_11b0, &args![target, 0xfa2_u32]).f32();
                reminder_fade(e, target, 0xfa2, start_x, end_x, seconds);
            }
        }
        if markers[3] != 0 {
            let wait_trait = e.mem.u32(0x011d_9e70);
            hud_trait(e, markers[3], wait_trait, 1);
        }
        hud_trait(e, markers[0], 0xfa3, 0);
        hud_trait(e, icons[0], 0xfa3, 0);
        let wait_trait = e.mem.u32(0x011d_9e70);
        hud_trait(e, markers[0], wait_trait, 0);
        markers[0] = 0;
    }
    for index in 1..=2 {
        if markers[index] == 0 {
            continue;
        }
        let wait_trait = e.mem.u32(0x011d_9e70);
        if e.call(0x00a0_11b0, &args![markers[index], wait_trait])
            .f64()
            != one
        {
            continue;
        }
        if e.call(0x00a0_11b0, &args![markers[index], 0xfa9_u32]).f64() != zero {
            continue;
        }
        let seconds = reminder_setting(e, 0x011d_2c94);
        let alpha = e.global::<f32>(0x011d_979c);
        reminder_fade(e, markers[index], 0xfa9, 0.0, alpha, seconds);
        let seconds = reminder_setting(e, 0x011d_2c94);
        let alpha = e.global::<f32>(0x011d_979c);
        reminder_fade(e, icons[index], 0xfa9, 0.0, alpha, seconds);
    }
}

// Translated from 0077c170 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::HandleCinematicFade` (Xbox PDB): clears compass markers
/// while the cinematic trait is set, then advances each queued text fade.
pub fn hud_main_menu_handle_cinematic_fade(e: &mut Engine, _this: Ptr<HUDMainMenu>, tile: Ptr) {
    let cinematic = e.call(0x00a0_11b0, &args![tile, 0xfb6_u32]).f64();
    if cinematic.partial_cmp(&e.global::<f64>(0x0101_2060)) != Some(std::cmp::Ordering::Greater) {
        return;
    }
    let hud = current_hud(e);
    let pool = e.mem.u32(hud + 0x5c);
    let mut cursor = e.call(0x0055_9450, &args![pool + 4]).u32();
    while cursor != 0 {
        let hud = current_hud(e);
        let pool = e.mem.u32(hud + 0x5c);
        let slot = reminder_advance(e, pool, &mut cursor, 0x0057_cbe0);
        let marker = e.mem.u32(slot);
        if marker != 0 {
            hud_set_float(e, marker, 0xfa9, 0.0);
            let wait_complete = e.call(0x00a0_1860, &args![0x0107_3724_u32]).u32();
            hud_trait(e, marker, wait_complete, 0);
        }
    }
    let mut cursor = e.call(0x0055_9450, &args![tile.addr() + 4]).u32();
    while cursor != 0 {
        let slot = reminder_advance(e, tile.addr(), &mut cursor, 0x0057_cbe0);
        let current = e.mem.u32(slot);
        if current != 0 {
            let start = e.call(0x00a0_11b0, &args![current, 0x1004_u32]).f64();
            if start > e.global::<f64>(0x0101_2060) {
                let now = e.call(0x0082_5c00, &args![0x011f_6394_u32]).u32();
                let start = e.call(0x00a0_11b0, &args![current, 0x1004_u32]).f64();
                if start < f64::from(now) {
                    let text = e.call(0x00a0_11f0, &args![current, 0xfc4_u32]).u32();
                    e.call(0x0070_7e10, &args![current, text]);
                    let seconds = reminder_setting(e, 0x011d_2238);
                    let setting = reminder_setting(e, 0x011d_9774);
                    let alpha = (f64::from(setting) * e.global::<f64>(0x0101_e568)) as f32;
                    reminder_fade(e, current, 0xfa9, 0.0, alpha, seconds);
                    hud_trait(e, current, 0x1004, 0);
                    continue;
                }
            }
        }
        if current != 0 {
            let end = e.call(0x00a0_11b0, &args![current, 0x1005_u32]).f64();
            if end > e.global::<f64>(0x0101_2060) {
                let now = e.call(0x0082_5c00, &args![0x011f_6394_u32]).u32();
                let end = e.call(0x00a0_11b0, &args![current, 0x1005_u32]).f64();
                if end < f64::from(now) {
                    let alpha = e.call(0x00a0_11b0, &args![current, 0xfa9_u32]).f64();
                    let start_alpha = if alpha > e.global::<f64>(0x0107_37c0) {
                        alpha as f32
                    } else {
                        e.global::<f32>(0x0105_478c)
                    };
                    let seconds = reminder_setting(e, 0x011d_4f30);
                    reminder_fade(e, current, 0xfa9, start_alpha, 0.0, seconds);
                    hud_trait(e, current, 0x1005, 0);
                    continue;
                }
            }
        }
        if current != 0
            && e.call(0x00a0_7f30, &args![current, 0xfa9_u32]).f64() == e.global::<f64>(0x0101_2060)
            && e.call(0x00a0_11b0, &args![current, 0xfa9_u32]).f64() == e.global::<f64>(0x0101_2060)
            && e.call(0x00a0_11b0, &args![current, 0x1005_u32]).f64()
                == e.global::<f64>(0x0101_2060)
        {
            hud_trait(e, current, 0xfa3, 0);
        }
    }
}

// Translated from 0077c440 (decompiled, FalloutNV.exe 1.4.0.525)
/// Enqueues a timed HUD value while the player's low 16-bit value is below
/// the game setting and the HUD's +0x204 byte is clear.
pub fn fn_0077c440(e: &mut Engine, value: u32) {
    let player = e.mem.u32(0x011d_ea3c);
    let current = e.call(0x0087_f9f0, &args![player]).u32() & 0xffff;
    let setting = e.call(0x0043_d4d0, &args![0x011d_0c60_u32]).u32();
    let hud = current_hud(e);
    if (current as i32) >= e.mem.i32(setting) || e.mem.u8(hud + 0x204) != 0 {
        return;
    }
    let record = e.call(0x0040_1000, &args![12_u32]).u32();
    e.mem.set_u32(record + 8, value);
    e.mem.set_u32(record, current);
    let player = e.mem.u32(0x011d_ea3c);
    let duration = e.vcall(player + 0xa4, 0xc, &args![0x18_u32]).f64();
    let duration = x87_to_int(e, duration);
    e.mem.set_u32(record + 4, duration as u32);
    e.with_stack(4, |e, slot| {
        e.mem.set_u32(slot.addr(), record);
        let hud = current_hud(e);
        e.call(0x005a_e3d0, &args![hud + 0x264, slot]);
    });
}

fn xp_tile(e: &Engine, this: Ptr<HUDMainMenu>, offset: u32) -> u32 {
    e.mem.u32(this.addr() + offset)
}

fn xp_player_experience(e: &mut Engine) -> f64 {
    let player = e.mem.u32(0x011d_ea3c);
    e.vcall(player + 0xa4, 0xc, &args![0x18_u32]).f64()
}

fn xp_start_gain(e: &mut Engine, this: Ptr<HUDMainMenu>, total: i32, in_combat: bool) {
    e.with_stack(24, |e, sound_handle| {
        e.call(0x0041_a250, &args![sound_handle]);
        e.with_stack(12, |e, temporary| {
            let audio = e.call(0x0045_3a70, &args![]).u32();
            let sound = e
                .call(
                    0x00ad_7550,
                    &args![audio, temporary, 0x0107_37e0_u32, 0x121_u32],
                )
                .u32();
            e.call(0x0041_8900, &args![sound_handle, sound]);
            e.call(0x0048_3710, &args![temporary]);
        });
        e.call(0x00ad_8830, &args![sound_handle, 0_u32]);
        let current_xp = xp_player_experience(e);
        let current_xp = x87_to_int(e, current_xp);
        let player = e.mem.u32(0x011d_ea3c);
        let level = e.call(0x0087_f9f0, &args![player]).u32() & 0xffff;
        e.mem.set_u32(0x011d_9e7c, level);
        e.mem.set_u32(0x011d_9e78, level.wrapping_add(1));
        let required_current = e.call(0x0064_8b50, &args![level]).i32();
        let next_level = e.mem.u32(0x011d_9e78);
        let required_next = e.call(0x0064_8b50, &args![next_level]).i32();
        if required_next < current_xp && in_combat {
            let tile = xp_tile(e, this, 0xe4);
            let max_trait = e.mem.u32(0x011d_9e80);
            e.call(0x00a0_11b0, &args![tile, max_trait]);
        }
        let target = if required_current == required_next {
            required_current as f32
        } else {
            let current_xp = (xp_player_experience(e) - f64::from(total)) as f32;
            let tile = xp_tile(e, this, 0xe4);
            let max_trait = e.mem.u32(0x011d_9e80);
            let max = e.call(0x00a0_11b0, &args![tile, max_trait]).f32();
            let tile = xp_tile(e, this, 0xe4);
            let min_trait = e.mem.u32(0x011d_9e84);
            let min = e.call(0x00a0_11b0, &args![tile, min_trait]).f32();
            e.call(
                0x004b_3ab0,
                &args![
                    min,
                    max,
                    required_current as f32,
                    required_next as f32,
                    current_xp
                ],
            )
            .f32()
        };
        let tile = xp_tile(e, this, 0xe4);
        let max_trait = e.mem.u32(0x011d_9e80);
        let max = e.call(0x00a0_11b0, &args![tile, max_trait]).f32();
        let tile = xp_tile(e, this, 0xe4);
        let min_trait = e.mem.u32(0x011d_9e84);
        let min = e.call(0x00a0_11b0, &args![tile, min_trait]).f32();
        let clamped = e.call(0x0040_4010, &args![target, min]).f32();
        let rounded = e.call(0x0040_ebd0, &args![clamped, max]).f32();
        let tile = xp_tile(e, this, 0xe4);
        hud_set_float(e, tile, 0xfa1, rounded);
        for offset in [0xd8, 0xdc, 0xe0] {
            let tile = xp_tile(e, this, offset);
            hud_trait(e, tile, 0xfa3, 1);
        }
        let text = e.call(0x0040_3df0, &args![0x011d_3bd4_u32]).u32();
        let tile = xp_tile(e, this, 0xe0);
        e.call(0x00a0_1350, &args![tile, 0xfc4_u32, text, 1_u32]);
        for offset in [0xe4, 0xe8, 0xec] {
            let tile = xp_tile(e, this, offset);
            hud_trait(e, tile, 0xfa3, 1);
        }
        for offset in [0xd8, 0xdc, 0xe0, 0xe4, 0xe8, 0xec] {
            let tile = xp_tile(e, this, offset);
            hud_set_float(e, tile, 0xfa9, 0.0);
        }
        e.with_stack(16, |e, buffer| {
            for (offset, format, value_address) in [
                (0xdc, 0x0107_34b0_u32, None),
                (0xe8, 0x0102_0774_u32, Some(0x011d_9e7c)),
                (0xec, 0x0102_0774_u32, Some(0x011d_9e78)),
            ] {
                let value = value_address.map_or(total as u32, |address| e.mem.u32(address));
                e.call(0x0040_6d00, &args![buffer, 16_u32, format, value]);
                let tile = xp_tile(e, this, offset);
                e.call(0x00a0_1350, &args![tile, 0xfc4_u32, buffer, 1_u32]);
            }
        });
        for offset in [0xd8, 0xdc, 0xe0, 0xe4, 0xe8, 0xec] {
            let alpha = e.global::<f32>(0x011d_979c);
            let seconds = e.global::<f32>(0x0101_62c0);
            let tile = xp_tile(e, this, offset);
            reminder_fade(e, tile, 0xfa9, 0.0, alpha, seconds);
        }
        e.call(0x0048_3710, &args![sound_handle]);
    });
}

fn xp_animate_current(
    e: &mut Engine,
    this: Ptr<HUDMainMenu>,
    fade_finished: bool,
    alpha_positive: bool,
    ready_to_level: bool,
    in_combat: bool,
) {
    let bar = xp_tile(e, this, 0xe4);
    let _fade_end = e.call(0x00a0_7f30, &args![bar, 0xfa1_u32]).f32();
    let bar = xp_tile(e, this, 0xe4);
    let min_trait = e.mem.u32(0x011d_9e84);
    let mut min = e.call(0x00a0_11b0, &args![bar, min_trait]).f32();
    let bar = xp_tile(e, this, 0xe4);
    let max_trait = e.mem.u32(0x011d_9e80);
    let max = e.call(0x00a0_11b0, &args![bar, max_trait]).f32();
    let bar = xp_tile(e, this, 0xe4);
    let position_finished = e.call(0x00a0_7fc0, &args![bar, 0xfa1_u32]).bool();
    let bar = xp_tile(e, this, 0xe4);
    let position_active = e.call(0x00a0_1230, &args![bar, 0x100f_u32]).bool();
    if !position_active && fade_finished && alpha_positive {
        let current_xp = xp_player_experience(e);
        let current_xp = x87_to_int(e, current_xp);
        if e.mem.u32(0x011d_9e7c) == e.mem.u32(0x011d_9e78) {
            let next = e.mem.u32(0x011d_9e78);
            e.mem.set_u32(0x011d_9e78, next.wrapping_add(1));
        }
        let level = e.mem.u32(0x011d_9e7c);
        let required_current = e.call(0x0064_8b50, &args![level]).i32();
        let next = e.mem.u32(0x011d_9e78);
        let required_next = e.call(0x0064_8b50, &args![next]).i32();
        let bar = xp_tile(e, this, 0xe4);
        hud_trait(e, bar, 0x100f, 1);
        if required_next < current_xp {
            let bar = xp_tile(e, this, 0xe4);
            let position = e.call(0x00a0_11b0, &args![bar, 0xfa1_u32]).f64();
            if f64::from(max) < position {
                let bar = xp_tile(e, this, 0xe4);
                hud_set_float(e, bar, 0xfa1, max);
            }
            let bar = xp_tile(e, this, 0xe4);
            let start = e.call(0x00a0_11b0, &args![bar, 0xfa1_u32]).f32();
            let bar = xp_tile(e, this, 0xe4);
            reminder_fade(e, bar, 0xfa1, start, max, 1.0);
        } else {
            let seconds = e.global::<f32>(0x0101_62c0);
            let clamped = e
                .call(0x0064_7b70, &args![current_xp, required_current])
                .i32();
            let clamped = e.call(0x004a_8f20, &args![clamped, required_next]).i32();
            let target = e
                .call(
                    0x004b_3ab0,
                    &args![
                        min,
                        max,
                        required_current as f32,
                        required_next as f32,
                        clamped as f32,
                    ],
                )
                .f32();
            let bar = xp_tile(e, this, 0xe4);
            let start = e.call(0x00a0_11b0, &args![bar, 0xfa1_u32]).f32();
            let bar = xp_tile(e, this, 0xe4);
            reminder_fade(e, bar, 0xfa1, start, target, seconds);
        }
    } else if !fade_finished || alpha_positive {
        if !position_finished && fade_finished && alpha_positive {
            let current_xp = xp_player_experience(e);
            let current_xp = x87_to_int(e, current_xp);
            let level = e.mem.u32(0x011d_9e7c);
            e.call(0x0064_8b50, &args![level]);
            let next = e.mem.u32(0x011d_9e78);
            let mut required_next = e.call(0x0064_8b50, &args![next]).i32();
            if ready_to_level {
                let bar = xp_tile(e, this, 0xe4);
                let position = e.call(0x00a0_11b0, &args![bar, 0xfa1_u32]).f64();
                if f64::from(max) == position {
                    let player = e.mem.u32(0x011d_ea3c);
                    if !e.call(0x005c_7870, &args![player]).bool() && !in_combat {
                        e.call(0x0071_7280, &args![0x15_u32]);
                    }
                }
            }
            if required_next < current_xp {
                let setting = e.call(0x0043_d4d0, &args![0x011d_0c60_u32]).u32();
                if (e.mem.u32(0x011d_9e78) as i32) < e.mem.i32(setting) && !in_combat {
                    let level = e.mem.u32(0x011d_9e7c);
                    e.mem.set_u32(0x011d_9e7c, level.wrapping_add(1));
                    let next = e.mem.u32(0x011d_9e78);
                    e.mem.set_u32(0x011d_9e78, next.wrapping_add(1));
                    let required_current = required_next;
                    let next = e.mem.u32(0x011d_9e78);
                    required_next = e.call(0x0064_8b50, &args![next]).i32();
                    let bar = xp_tile(e, this, 0xe4);
                    hud_set_float(e, bar, 0xfa1, min);
                    e.with_stack(4, |e, buffer| {
                        let level = e.mem.u32(0x011d_9e7c);
                        e.call(0x0040_6d00, &args![buffer, 3_u32, 0x0102_0774_u32, level]);
                        let tile = xp_tile(e, this, 0xe8);
                        e.call(0x00a0_1350, &args![tile, 0xfc4_u32, buffer, 1_u32]);
                        let next = e.mem.u32(0x011d_9e78);
                        e.call(0x0040_6d00, &args![buffer, 3_u32, 0x0102_0774_u32, next]);
                        let tile = xp_tile(e, this, 0xec);
                        e.call(0x00a0_1350, &args![tile, 0xfc4_u32, buffer, 1_u32]);
                    });
                    if required_next < current_xp {
                        if max < min {
                            min = max;
                        }
                        let bar = xp_tile(e, this, 0xe4);
                        reminder_fade(e, bar, 0xfa1, min, max, 1.0);
                    } else {
                        let seconds = e.global::<f32>(0x0101_62c0);
                        let target = e
                            .call(
                                0x004b_3ab0,
                                &args![
                                    min,
                                    max,
                                    required_current as f32,
                                    required_next as f32,
                                    current_xp as f32,
                                ],
                            )
                            .f32();
                        let bar = xp_tile(e, this, 0xe4);
                        reminder_fade(e, bar, 0xfa1, min, target, seconds);
                    }
                }
            }
        } else {
            let bar = xp_tile(e, this, 0xe4);
            if e.call(0x00a0_7fc0, &args![bar, 0xfa1_u32]).bool() {
                let bar = xp_tile(e, this, 0xe4);
                if e.call(0x00a0_1230, &args![bar, 0x100f_u32]).bool() {
                    let meter = xp_tile(e, this, 0xd8);
                    if e.call(0x00a0_7fc0, &args![meter, 0xfa9_u32]).bool() {
                        let meter = xp_tile(e, this, 0xd8);
                        if e.call(0x00a0_11b0, &args![meter, 0xfa9_u32]).f64()
                            > e.global::<f64>(0x0101_2060)
                        {
                            let bar = xp_tile(e, this, 0xe4);
                            hud_trait(e, bar, 0x100f, 0);
                            for offset in [0xd8, 0xdc, 0xe0, 0xe4, 0xe8, 0xec] {
                                let seconds = e.global::<f32>(0x0101_62c0);
                                let tile = xp_tile(e, this, offset);
                                let start = e.call(0x00a0_11b0, &args![tile, 0xfa9_u32]).f32();
                                let tile = xp_tile(e, this, offset);
                                reminder_fade(e, tile, 0xfa9, start, 0.0, seconds);
                            }
                        }
                    }
                }
            }
        }
    } else {
        for offset in [0xd8, 0xdc, 0xe0, 0xe4, 0xe8, 0xec] {
            let tile = hud_field(e, offset);
            hud_trait(e, tile, 0xfa3, 0);
        }
        if e.mem.u32(0x011a_010c) & 8 != 0 && e.mem.u8(this.addr() + 0x204) == 0 {
            e.mem.set_u8(this.addr() + 0x204, 1);
            let popup = xp_tile(e, this, 0xf0);
            hud_trait(e, popup, 0x100e, 0);
        }
    }
}

// Translated from 0077c4e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::UpdateXPMeter` (Xbox PDB): controls XP meter visibility,
/// level-up prompts, and queued XP gain animations.
// Translation incomplete: 0077c4e0 - compiler SEH unwinding.
pub fn hud_main_menu_update_xp_meter(e: &mut Engine, this: Ptr<HUDMainMenu>) {
    if e.mem.u32(0x011d_9e88) & 1 == 0 {
        let flags = e.mem.u32(0x011d_9e88);
        e.mem.set_u32(0x011d_9e88, flags | 1);
        let trait_id = e.call(0x00a0_0940, &args![0x0107_37fc_u32, -1_i32]).u32();
        e.mem.set_u32(0x011d_9e84, trait_id);
    }
    if e.mem.u32(0x011d_9e88) & 2 == 0 {
        let flags = e.mem.u32(0x011d_9e88);
        e.mem.set_u32(0x011d_9e88, flags | 2);
        let trait_id = e.call(0x00a0_0940, &args![0x0107_37f4_u32, -1_i32]).u32();
        e.mem.set_u32(0x011d_9e80, trait_id);
    }
    let player = e.mem.u32(0x011d_ea3c);
    let in_combat = e.call(0x0095_3c20, &args![player]).i32() > 0;
    let player = e.mem.u32(0x011d_ea3c);
    let progression = e.call(0x0046_4e10, &args![player]).u32();
    let ready_to_level = e.call(0x008d_51f0, &args![progression]).bool();
    let menu_allowed = !e.call(0x0070_2360, &args![]).bool()
        || e.call(0x0070_23c0, &args![]).u32() == 0x3f0
        || e.call(0x0070_23c0, &args![]).u32() == 0x41f
        || e.call(0x0070_23c0, &args![]).u32() == 0x421
        || e.call(0x0070_23c0, &args![]).u32() == 0x3f1;
    let visible = menu_allowed
        && e.call(0x0044_ddc0, &args![0x011f_2250_u32]).i32() != 4
        && !e.call(0x0070_1740, &args![this.addr() + 0x1c8]).bool();
    if !visible {
        for offset in [0xd8, 0xdc, 0xe0, 0xe4, 0xe8, 0xec] {
            let tile = hud_field(e, offset);
            hud_trait(e, tile, 0xfa3, 0);
        }
        let tile = hud_field(e, 0xd8);
        hud_set_float(e, tile, 0xfa9, 0.0);
        let tile = hud_field(e, 0xd8);
        e.call(0x00a0_7dc0, &args![tile, 0xfa9_u32]);
        let flags = e.mem.u32(0x011a_010c);
        e.mem.set_u32(0x011a_010c, flags | 1);
        e.mem.set_u8(this.addr() + 0x204, 0);
        return;
    }
    if e.global::<f32>(0x011d_979c) == 0.0 {
        e.mem.set_u8(this.addr() + 0x204, 1);
        return;
    }
    let meter = e.mem.u32(this.addr() + 0xd8);
    let meter_alpha = e.call(0x00a0_11b0, &args![meter, 0xfa9_u32]).f32();
    let mut alpha_positive = f64::from(meter_alpha) > e.global::<f64>(0x0101_2060);
    let meter = e.mem.u32(this.addr() + 0xd8);
    let fade_finished = e.call(0x00a0_7fc0, &args![meter, 0xfa9_u32]).bool();
    if e.mem.u32(0x011a_010c) & 1 == 0 {
        let defer_level_up = e.mem.u32(0x011a_010c) & 2 == 0
            || e.mem.u32(0x011a_010c) & 8 != 0
            || alpha_positive
            || !fade_finished
            || {
                let player = e.mem.u32(0x011d_ea3c);
                e.call(0x005c_7870, &args![player]).bool()
            };
        if defer_level_up {
            if e.mem.u32(0x011a_010c) & 4 != 0 {
                let hidden = e.mem.u32(0x011a_010c) & 8 != 0;
                // 0077d933 / 0077d94f: both reads take ECX = [this+0xf0], the popup tile.
                let popup = e.mem.u32(this.addr() + 0xf0);
                let finished = e.call(0x00a0_7fc0, &args![popup, 0xfa9_u32]).bool();
                let popup = e.mem.u32(this.addr() + 0xf0);
                let zero_alpha = e.call(0x00a0_11b0, &args![popup, 0xfa9_u32]).f64()
                    == e.global::<f64>(0x0101_2060);
                if !hidden {
                    if !in_combat {
                        if !alpha_positive {
                            e.mem.set_u32(0x011a_010c, 2);
                        }
                    } else {
                        let flags = e.mem.u32(0x011a_010c);
                        e.mem.set_u32(0x011a_010c, flags | 1);
                    }
                } else if e.mem.u8(this.addr() + 0x204) == 0 && finished && zero_alpha {
                    e.mem.set_u8(this.addr() + 0x204, 1);
                    e.mem.set_u32(0x011a_010c, 8);
                    let popup = e.mem.u32(this.addr() + 0xf0);
                    hud_trait(e, popup, 0x100e, 0);
                }
            }
        } else {
            let popup = e.mem.u32(this.addr() + 0xf0);
            let finished = e.call(0x00a0_7fc0, &args![popup, 0xfa9_u32]).bool();
            let popup = e.mem.u32(this.addr() + 0xf0);
            let zero_alpha =
                e.call(0x00a0_11b0, &args![popup, 0xfa9_u32]).f64() == e.global::<f64>(0x0101_2060);
            let can_start = finished
                && zero_alpha
                && {
                    let popup = e.mem.u32(this.addr() + 0xf0);
                    !e.call(0x00a0_1230, &args![popup, 0x100e_u32]).bool()
                }
                && {
                    let player = e.mem.u32(0x011d_ea3c);
                    e.call(0x0095_3c20, &args![player]).i32() == 0
                };
            if can_start {
                e.with_stack(12, |e, temporary| {
                    let audio = e.call(0x0045_3a70, &args![]).u32();
                    let handle = e
                        .call(
                            0x00ad_7550,
                            &args![audio, temporary, 0x0107_37d0_u32, 0x121_u32],
                        )
                        .u32();
                    e.call(0x00ad_8830, &args![handle, 0_u32]);
                    e.call(0x0048_3710, &args![temporary]);
                });
                let popup = e.mem.u32(this.addr() + 0xf0);
                hud_set_float(e, popup, 0xfa9, 0.0);
                let alpha = e.global::<f32>(0x011d_979c);
                let duration = e.global::<f32>(0x0101_6248);
                let popup = e.mem.u32(this.addr() + 0xf0);
                reminder_fade(e, popup, 0xfa9, 0.0, alpha, duration);
                let text = e.call(0x0040_3df0, &args![0x011d_4324_u32]).u32();
                let popup = e.mem.u32(this.addr() + 0xf0);
                e.call(0x00a0_1350, &args![popup, 0xfc4_u32, text, 1_u32]);
                let popup = e.mem.u32(this.addr() + 0xf0);
                hud_trait(e, popup, 0xfa3, 1);
                let now = e.call(0x0082_5c00, &args![0x011f_6394_u32]).u32();
                let popup = e.mem.u32(this.addr() + 0xf0);
                e.call(0x0071_5c60, &args![popup, 0x100d_u32, now]);
                let popup = e.mem.u32(this.addr() + 0xf0);
                hud_trait(e, popup, 0x100e, 1);
            } else {
                let now = e.call(0x0082_5c00, &args![0x011f_6394_u32]).u32();
                let popup = e.mem.u32(this.addr() + 0xf0);
                let started = e.call(0x00a0_11b0, &args![popup, 0x100d_u32]).f64();
                if f64::from(now) - started > e.global::<f64>(0x0107_37c8)
                    && !zero_alpha
                    && finished
                {
                    let now = e.call(0x0082_5c00, &args![0x011f_6394_u32]).u32();
                    let popup = e.mem.u32(this.addr() + 0xf0);
                    e.call(
                        0x0071_5c60,
                        &args![popup, 0x100d_u32, now.wrapping_add(2000)],
                    );
                    let alpha = e.global::<f32>(0x011d_979c);
                    let duration = e.global::<f32>(0x0101_6248);
                    let popup = e.mem.u32(this.addr() + 0xf0);
                    reminder_fade(e, popup, 0xfa9, alpha, 0.0, duration);
                } else if e.mem.u8(this.addr() + 0x204) == 0 && finished && zero_alpha {
                    e.mem.set_u8(this.addr() + 0x204, 1);
                    e.mem.set_u32(0x011a_010c, 8);
                    let popup = e.mem.u32(this.addr() + 0xf0);
                    hud_trait(e, popup, 0x100e, 0);
                }
            }
        }
        return;
    }
    if !alpha_positive && fade_finished && !e.call(0x0082_56d0, &args![this.addr() + 0x264]).bool()
    {
        let mut cursor = this.addr() + 0x264;
        let mut total = 0_i32;
        while cursor != 0 && !e.call(0x0082_56d0, &args![cursor]).bool() {
            let slot = e.call(0x0068_15c0, &args![cursor]).u32();
            let record = e.mem.u32(slot);
            total = total.wrapping_add(e.mem.i32(record + 8));
            cursor = e.call(0x0072_6070, &args![cursor]).u32();
        }
        loop {
            let slot = e.call(0x0068_15c0, &args![this.addr() + 0x264]).u32();
            if e.mem.u32(slot) == 0 {
                break;
            }
            let slot = e.call(0x0068_15c0, &args![this.addr() + 0x264]).u32();
            let record = e.mem.u32(slot);
            e.call(0x0040_1030, &args![record]);
            e.call(0x0063_f7b0, &args![this.addr() + 0x264]);
        }
        if total > 0 && e.mem.u8(this.addr() + 0x204) == 0 {
            xp_start_gain(e, this, total, in_combat);
            alpha_positive = true;
        }
    } else {
        xp_animate_current(
            e,
            this,
            fade_finished,
            alpha_positive,
            ready_to_level,
            in_combat,
        );
    }
    if ready_to_level {
        if !in_combat && !alpha_positive && e.mem.u32(0x011a_010c) & 8 == 0 {
            e.mem.set_u32(0x011a_010c, 2);
        } else {
            let flags = e.mem.u32(0x011a_010c);
            e.mem.set_u32(0x011a_010c, flags | 4);
        }
    }
}

// Translated from 0077da30 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::ClearXPMeterFinishedMoving` (Xbox PDB): clears the HUD's
/// XP animation byte and resets the shared XP state to one.
pub fn hud_main_menu_clear_xp_meter_finished_moving(e: &mut Engine) {
    let hud = current_hud(e);
    if hud != 0 {
        let hud = current_hud(e);
        e.mem.set_u8(hud + 0x204, 0);
    }
    e.mem.set_u32(0x011a_010c, 1);
}

fn hotkey_wheel(_e: &Engine, this: Ptr<HUDMainMenu>) -> u32 {
    this.addr() + 0x1c8
}

fn hotkey_time(e: &mut Engine) -> u32 {
    e.call(0x0082_5c00, &args![0x011f_6394_u32]).u32()
}

fn hotkey_timeout_elapsed(e: &mut Engine, setting_address: u32) -> bool {
    let elapsed = hotkey_time(e).wrapping_sub(e.mem.u32(0x011d_9e8c));
    let setting = e.call(0x0040_3e20, &args![setting_address]).u32();
    f64::from(e.mem.f32(setting)) * e.global::<f64>(0x0101_7b70) < f64::from(elapsed)
}

fn hotkey_pressed(e: &mut Engine, index: i32, mode: u32) -> bool {
    let input = e.mem.u32(0x011d_ea0c);
    // 0077def9 / 0077e15e: `00877720` takes no stack word (plain RET); the pushed
    // control and mode are the two words of `00a24660` (RET 8).
    let key = e.call(0x0087_7720, &args![input]).u32();
    e.call(0x00a2_4660, &args![key, index + 0x11, mode]).bool()
}

fn hotkey_show_icon(e: &mut Engine, this: Ptr<HUDMainMenu>, icon: u32) {
    if icon == 0 {
        let trait_id = e.call(0x00a0_1860, &args![0x0107_3814_u32]).u32();
        let tile = xp_tile(e, this, 0xc8);
        hud_trait(e, tile, trait_id, 0);
    } else {
        let trait_id = e.call(0x00a0_1860, &args![0x0107_3804_u32]).u32();
        let tile = xp_tile(e, this, 0xc8);
        e.call(0x00a0_1350, &args![tile, trait_id, icon, 1_u32]);
        let trait_id = e.call(0x00a0_1860, &args![0x0107_3814_u32]).u32();
        let tile = xp_tile(e, this, 0xc8);
        hud_trait(e, tile, trait_id, 1);
    }
}

fn hotkey_item_name(e: &mut Engine, item: u32, buffer: Ptr) {
    if e.call(0x0072_6070, &args![item]).i32() < 2 {
        let name = e.call(0x004b_e2d0, &args![item]).u32();
        e.call(0x0040_6d30, &args![buffer, 0x104_u32, name]);
    } else {
        let count = e.call(0x0072_6070, &args![item]).u32();
        let name = e.call(0x004b_e2d0, &args![item]).u32();
        e.call(
            0x0040_6d00,
            &args![buffer, 0x104_u32, 0x0106_eb00_u32, name, count],
        );
    }
}

// Translated from 0077da60 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::UpdateHotkeysWheel` (Xbox PDB): updates controller and
/// keyboard hotkey wheel visibility, highlighted item, and icon traits.
// Translation incomplete: 0077da60 - compiler cookie check.
pub fn hud_main_menu_update_hotkeys_wheel(e: &mut Engine, this: Ptr<HUDMainMenu>) {
    let interface = e.call(0x004b_7210, &args![]).u32();
    let menu_blocked = e.call(0x0070_23a0, &args![interface]).bool();
    let player = e.mem.u32(0x011d_ea3c);
    let player_blocked = if menu_blocked {
        false
    } else {
        e.vcall(player, 0x22c, &args![0_u32]).bool()
    };
    let input_blocked = if menu_blocked || player_blocked {
        false
    } else {
        let player = e.mem.u32(0x011d_ea3c);
        e.call(0x005a_03f0, &args![player, 4_u32]).bool()
    };
    if menu_blocked || player_blocked || input_blocked {
        e.mem.set_u32(0x011d_9e8c, 0);
        let wheel = hotkey_wheel(e, this);
        e.call(0x0070_1760, &args![wheel, 0_u32]);
        let label = xp_tile(e, this, 0xcc);
        hud_trait(e, label, 0xfa3, 0);
        return;
    }
    if !e.call(0x004b_71d0, &args![]).bool() {
        let wheel = hotkey_wheel(e, this);
        let mut index = 0;
        while index < e.call(0x006e_4ba0, &args![wheel]).i32() {
            if hotkey_pressed(e, index, 1) {
                if index == 1 {
                    e.call(0x0061_cc40, &args![wheel, index]);
                } else {
                    let player = e.mem.u32(0x011d_ea3c);
                    let inventory = e.call(0x004b_f220, &args![player]).u32();
                    let now = hotkey_time(e);
                    e.mem.set_u32(0x011d_9e8c, now);
                    e.call(0x0070_1bd0, &args![wheel, index]);
                    let item = if inventory == 0 {
                        0
                    } else {
                        e.call(0x004b_f4b0, &args![inventory, index]).u32()
                    };
                    let mut icon = 0;
                    if item == 0 {
                        let label = xp_tile(e, this, 0xcc);
                        e.call(
                            0x00a0_1350,
                            &args![label, 0xfc4_u32, 0x0102_0770_u32, 1_u32],
                        );
                    } else {
                        icon = e.call(0x004b_e200, &args![item, 0_u32]).u32();
                        e.with_stack(0x104, |e, buffer| {
                            hotkey_item_name(e, item, buffer);
                            let label = xp_tile(e, this, 0xcc);
                            e.call(0x00a0_1350, &args![label, 0xfc4_u32, buffer, 1_u32]);
                        });
                    }
                    hotkey_show_icon(e, this, icon);
                    if item != 0 {
                        e.call(0x0044_59e0, &args![item, 1_u32]);
                    }
                }
            } else if hotkey_pressed(e, index, 0) {
                if !e.call(0x0070_1740, &args![wheel]).bool()
                    && e.mem.i32(0x011d_9e8c) > 0
                    && hotkey_timeout_elapsed(e, 0x011d_2d84)
                    && index != 1
                {
                    e.call(0x0070_1760, &args![wheel, 1_u32]);
                    let label = xp_tile(e, this, 0xcc);
                    hud_trait(e, label, 0xfa3, 1);
                    for offset in [0xd8, 0xdc, 0xe0, 0xe4, 0xe8, 0xec] {
                        let tile = xp_tile(e, this, offset);
                        hud_trait(e, tile, 0xfa3, 0);
                    }
                }
                if e.call(0x0070_1740, &args![wheel]).bool() {
                    let now = hotkey_time(e);
                    e.mem.set_u32(0x011d_9e8c, now);
                }
            } else if hotkey_pressed(e, index, 2) {
                e.call(0x0070_1e80, &args![wheel]);
                if !e.call(0x0070_1740, &args![wheel]).bool() {
                    e.mem.set_u32(0x011d_9e8c, 0);
                }
            }
            index += 1;
        }
        if e.call(0x0070_1740, &args![wheel]).bool()
            && e.mem.i32(0x011d_9e8c) > 0
            && hotkey_timeout_elapsed(e, 0x011d_279c)
        {
            let label = xp_tile(e, this, 0xcc);
            hud_trait(e, label, 0xfa3, 0);
            e.call(0x0070_1760, &args![wheel, 0_u32]);
            e.mem.set_u32(0x011d_9e8c, 0);
        }
        return;
    }
    let wheel = hotkey_wheel(e, this);
    let highlighted = e.call(0x0070_1bd0, &args![wheel, -1_i32]).i32();
    if e.mem.u8(0x011d_9e90) == 0 && highlighted >= 0 {
        e.call(0x0070_17b0, &args![wheel]);
        e.mem.set_u8(0x011d_9e90, 1);
        let now = hotkey_time(e);
        e.mem.set_u32(0x011d_9e8c, now);
    }
    if e.mem.u8(0x011d_9e90) == 0 {
        return;
    }
    if highlighted < 0 {
        e.call(0x0070_1e80, &args![wheel]);
        e.call(0x0070_1760, &args![wheel, 0_u32]);
        let label = xp_tile(e, this, 0xcc);
        hud_trait(e, label, 0xfa3, 0);
        e.mem.set_u32(0x011d_9e8c, 0);
        e.mem.set_u8(0x011d_9e90, 0);
        e.mem.set_u32(0x011a_0124, u32::MAX);
        return;
    }
    if e.mem.i32(0x011d_9e8c) > 0
        && hotkey_timeout_elapsed(e, 0x011d_2d84)
        && !e.call(0x0070_1740, &args![wheel]).bool()
    {
        e.call(0x0070_1760, &args![wheel, 1_u32]);
        for offset in [0xd8, 0xdc, 0xe0, 0xe4, 0xe8, 0xec] {
            let tile = xp_tile(e, this, offset);
            hud_trait(e, tile, 0xfa3, 0);
        }
    }
    if e.call(0x0070_1740, &args![wheel]).bool() && highlighted != e.mem.i32(0x011a_0124) {
        let player = e.mem.u32(0x011d_ea3c);
        let inventory = e.call(0x004b_f220, &args![player]).u32();
        let item = if inventory == 0 {
            0
        } else {
            e.call(0x004b_f4b0, &args![inventory, highlighted]).u32()
        };
        let mut icon = 0;
        if item == 0 {
            let label = xp_tile(e, this, 0xcc);
            hud_trait(e, label, 0xfa3, 0);
        } else {
            icon = e.call(0x004b_e200, &args![item, 0_u32]).u32();
            e.with_stack(0x104, |e, buffer| {
                hotkey_item_name(e, item, buffer);
                let label = xp_tile(e, this, 0xcc);
                e.call(0x00a0_1350, &args![label, 0xfc4_u32, buffer, 1_u32]);
            });
            let label = xp_tile(e, this, 0xcc);
            hud_trait(e, label, 0xfa3, 1);
        }
        hotkey_show_icon(e, this, icon);
        e.mem.set_u32(0x011a_0124, highlighted as u32);
        if item != 0 {
            e.call(0x0044_59e0, &args![item, 1_u32]);
        }
    }
}

// Translated from 0077e430 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the HUD message queue, then sets opacity for the visible subset of
/// indexed HUD tiles. The excluded indices come from the PC switch table.
pub fn fn_0077e430(e: &mut Engine, opacity: f32) {
    let hud = current_hud(e);
    if hud == 0 {
        return;
    }
    e.call(0x0077_5920, &args![hud]);
    for index in 1..0x3b_u32 {
        if matches!(index, 0x0f..=0x14 | 0x18..=0x1b | 0x2c | 0x32 | 0x34..=0x3a) {
            continue;
        }
        let alpha = (f64::from(opacity) * e.global::<f64>(0x0101_e568)) as f32;
        let hud = current_hud(e);
        let tile = e.mem.u32(hud + index * 4 + 0x28);
        e.call(0x00a0_12d0, &args![tile, 0xfa9_u32, alpha, 1_u32]);
    }
}

// Translated from 0077e4f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Updates the breath indicator's fade and its linked 3D controller phase
/// from the player's process values.
pub fn fn_0077e4f0(e: &mut Engine, this: Ptr<HUDMainMenu>) {
    let player = e.mem.u32(0x011d_ea3c);
    let process = e.call(0x008d_6f30, &args![player]).u32();
    let player = e.mem.u32(0x011d_ea3c);
    let state = e.vcall(player, 0x1f4, &args![]).u32();
    let player = e.mem.u32(0x011d_ea3c);
    let state_breath = e.mem.f32(state + 8);
    let breath = e
        .call(0x0088_5560, &args![player, state_breath, process])
        .f32();
    let _breathing = f64::from(breath) >= e.global::<f64>(0x0101_6408);
    let full = f64::from(breath) >= e.global::<f64>(0x0107_3828);
    let player = e.mem.u32(0x011d_ea3c);
    let endurance = e.call(0x008b_e7a0, &args![player]).f64();
    let endurance = e.call(0x00ec_62c0, &args![endurance]).u32();
    let breath_time = e.call(0x0064_8a10, &args![endurance]).f32();
    let player = e.mem.u32(0x011d_ea3c);
    let saved = e.call(0x008d_8520, &args![player]).u32();
    let phase_scale = if saved == 0 {
        e.global::<f64>(0x0101_7a40) as f32
    } else {
        let player = e.mem.u32(0x011d_ea3c);
        let saved = e.call(0x008d_8520, &args![player]).u32();
        let elapsed = e.vcall(saved, 0x300, &args![]).f64();
        (elapsed / f64::from(breath_time)) as f32
    };
    let tile = e.mem.u32(this.addr() + 0x148);
    let alpha = e.call(0x00a0_11b0, &args![tile, 0xfa9_u32]).f32();
    if full {
        if alpha == 0.0 {
            let start = 0.0_f32;
            let end = e.global::<f32>(0x011d_979c);
            let seconds = e.global::<f32>(0x0101_6248);
            let tile = e.mem.u32(this.addr() + 0x148);
            reminder_fade(e, tile, 0xfa9, start, end, seconds);
        }
    } else if f64::from(alpha) > e.global::<f64>(0x0101_2060) {
        let tile = e.mem.u32(this.addr() + 0x148);
        if e.call(0x00a0_7fc0, &args![tile, 0xfa9_u32]).bool() {
            let start = e.global::<f32>(0x011d_979c);
            let seconds = e.global::<f32>(0x0101_6248);
            let tile = e.mem.u32(this.addr() + 0x148);
            reminder_fade(e, tile, 0xfa9, start, 0.0, seconds);
        }
    }
    if e.mem.u32(this.addr() + 0x200) != 0 && f64::from(alpha) > e.global::<f64>(0x0101_2060) {
        e.with_stack(12, |e, vector| {
            e.call(0x0043_d410, &args![vector, 0.0_f32, 1_u32, 0_u32]);
            let controller = e.mem.u32(this.addr() + 0x200);
            // 0077e6d4: `00508100` takes no stack word (plain RET); the pushed 0 is
            // the last word of `00a328b0` (RET 8) below.
            let from = e.call(0x0050_8100, &args![controller]).f64();
            let controller = e.mem.u32(this.addr() + 0x200);
            let destination = e.call(0x0075_9450, &args![controller]).f64();
            let controller = e.mem.u32(this.addr() + 0x200);
            let current = e.call(0x0050_8100, &args![controller]).f64();
            let phase = ((destination - current) * f64::from(phase_scale) + from) as f32;
            let controller = e.mem.u32(this.addr() + 0x200);
            e.call(0x00a3_28b0, &args![controller, phase, 0_u32]);
            let tile = e.mem.u32(this.addr() + 0x148);
            let sequence_list = e.call(0x0056_c7f0, &args![tile]).u32();
            let sequence = e.call(0x0043_b4a0, &args![sequence_list, 0_u32]).u32();
            e.call(0x00a5_9c60, &args![sequence, vector]);
        });
    }
}

// Translated from 0077e750 (decompiled, FalloutNV.exe 1.4.0.525)
/// Scans running package targets for a nearby actor and displays the linked
/// target indicator; unused indicators fade away after the scan.
pub fn fn_0077e750(e: &mut Engine, this: Ptr<HUDMainMenu>) {
    let indicators = e.mem.u32(this.addr() + 0x14c);
    let mut cursor = e.call(0x0055_9450, &args![indicators + 4]).u32();
    let package = e.call(0x0071_7e50, &args![0x011e_0e80_u32]).u32();
    let mut index = e.call(0x005d_e0f0, &args![package, 0_u32]).u32();
    loop {
        let package = e.call(0x0071_7e50, &args![0x011e_0e80_u32]).u32();
        let count = e.call(0x005b_e5c0, &args![package, 0_u32]).u32();
        if index >= count {
            break;
        }
        let package = e.call(0x0071_7e50, &args![0x011e_0e80_u32]).u32();
        let actor = e.call(0x0096_8670, &args![package, index]).u32();
        if actor != 0
            && !e.call(0x0057_6d30, &args![actor]).bool()
            && !e.call(0x0044_0d80, &args![actor]).bool()
            && !e.call(0x0044_0da0, &args![actor]).bool()
            && e.call(0x0040_1170, &args![actor]).u32() == 0x3e
            && e.vcall(actor, 0x224, &args![]).bool()
            && !e.call(0x0077_eac0, &args![actor]).bool()
        {
            let process = e.call(0x0041_81e0, &args![actor]).u32();
            if e.call(0x004f_d3c0, &args![process]).u32() != 0 {
                let process = e.call(0x0041_81e0, &args![actor]).u32();
                let vertical = e.call(0x0045_cd80, &args![process]).f64();
                if vertical == e.global::<f64>(0x0101_2060) {
                    let player = e.mem.u32(0x011d_ea3c);
                    let player_position = e.vcall(player, 0x1f4, &args![]).u32();
                    let distance = e.with_stack(12, |e, vector| {
                        let actor_position = e.vcall(actor, 0x1f4, &args![]).u32();
                        let normal = e
                            .call(0x0043_9ef0, &args![actor_position, vector, player_position])
                            .u32();
                        e.call(0x0045_7990, &args![normal]).f64()
                    });
                    let setting = e.call(0x0040_3e20, &args![0x011d_45dc_u32]).u32();
                    if distance < f64::from(e.mem.f32(setting)) && cursor != 0 {
                        let indicators = e.mem.u32(this.addr() + 0x14c);
                        let slot = reminder_advance(e, indicators, &mut cursor, 0x0057_cbe0);
                        let tile = e.mem.u32(slot);
                        if e.call(0x00a0_7fc0, &args![tile, 0xfa9_u32]).bool()
                            && e.call(0x00a0_11b0, &args![tile, 0xfa9_u32]).f64()
                                == e.global::<f64>(0x0101_2060)
                        {
                            let seconds = e.global::<f32>(0x0101_6248);
                            let setting = e.call(0x0040_3e20, &args![0x011d_9774_u32]).u32();
                            let end = (f64::from(e.mem.f32(setting)) * e.global::<f64>(0x0101_e568))
                                as f32;
                            reminder_fade(e, tile, 0xfa9, 0.0, end, seconds);
                        }
                        let player = e.mem.u32(0x011d_ea3c);
                        let player_position = e.vcall(player, 0x1f4, &args![]).u32();
                        let target_angle = e.with_stack(12, |e, vector| {
                            let actor_position = e.vcall(actor, 0x1f4, &args![]).u32();
                            let normal = e
                                .call(0x0043_9ef0, &args![actor_position, vector, player_position])
                                .u32();
                            e.call(0x004b_13c0, &args![normal]).f32()
                        });
                        let player_angle = e.with_stack(12, |e, vector| {
                            let face = e.call(0x0045_c670, &args![]).u32();
                            let animation = e.call(0x0066_29f0, &args![face]).u32();
                            let direction = e.call(0x0045_bba0, &args![animation, vector]).u32();
                            e.call(0x004b_13c0, &args![direction]).f32()
                        });
                        let difference = player_angle - target_angle;
                        let controller = e.call(0x0056_c7f0, &args![tile]).u32();
                        e.call(
                            0x0064_04b0,
                            &args![controller, difference, 0.0_f32, 1.0_f32, 0.0_f32],
                        );
                        e.with_stack(12, |e, vector| {
                            e.call(0x0043_d410, &args![vector, 0.0_f32, 1_u32, 0_u32]);
                            let controller = e.call(0x0056_c7f0, &args![tile]).u32();
                            e.call(0x00a5_9c60, &args![controller, vector]);
                        });
                    }
                }
            }
        }
        index = index.wrapping_add(1);
    }
    while cursor != 0 {
        let indicators = e.mem.u32(this.addr() + 0x14c);
        let slot = reminder_advance(e, indicators, &mut cursor, 0x0057_cbe0);
        let tile = e.mem.u32(slot);
        if e.call(0x00a0_7fc0, &args![tile, 0xfa9_u32]).bool()
            && e.call(0x00a0_11b0, &args![tile, 0xfa9_u32]).f64() > e.global::<f64>(0x0101_2060)
        {
            let seconds = e.global::<f32>(0x0101_6248);
            let start = e.call(0x00a0_11b0, &args![tile, 0xfa9_u32]).f32();
            reminder_fade(e, tile, 0xfa9, start, 0.0, seconds);
        }
    }
}

// Translated from 0077eac0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Checks whether the target's owning location fails the global membership
/// test; the PC uses the negated Boolean result.
pub fn fn_0077eac0(e: &mut Engine, target: Ptr) -> bool {
    let location = e.call(0x0084_e3a0, &args![target]).u32();
    let registry = e.mem.u32(0x011c_3f2c);
    !e.call(0x0046_9860, &args![registry, location]).bool()
}

// Translated from 0077eaf0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::ShowCrippledGuy` (Xbox PDB): chooses the face and six limb
/// textures from player values, then animates their HUD tiles.
// Translation incomplete: 0077eaf0 - compiler cookie check.
pub fn hud_main_menu_show_crippled_guy(e: &mut Engine, this: Ptr<HUDMainMenu>) {
    let mut face = 0_i32;
    let player = e.mem.u32(0x011d_ea3c);
    let mut node = e.vcall(player + 0x94, 8, &args![]).u32();
    while node != 0 {
        let entry = e.call(0x0068_15c0, &args![node]).u32();
        if e.mem.u32(entry) == 0 {
            break;
        }
        let entry = e.call(0x0068_15c0, &args![node]).u32();
        let record = e.mem.u32(entry);
        if e.call(0x0055_b980, &args![record]).i32() == 10 {
            face = 10;
        }
        node = e.call(0x0072_6070, &args![node]).u32();
    }
    if face == 0 {
        let player = e.mem.u32(0x011d_ea3c);
        let max = e.vcall(player + 0xa4, 0x20, &args![0x10_u32]).f32();
        let player = e.mem.u32(0x011d_ea3c);
        let current = e.vcall(player + 0xa4, 0xc, &args![0x10_u32]).f64();
        let difference = (f64::from(max) - current) as f32;
        let scaled = f64::from(difference) / f64::from(max) * e.global::<f64>(0x0102_0998);
        face = x87_to_int(e, scaled).clamp(0, 4);
    }
    e.with_stack(0x104, |e, buffer| {
        e.call(
            0x0040_6d00,
            &args![buffer, 0x104_u32, 0x0107_3868_u32, face],
        );
        let tile = e.mem.u32(this.addr() + 0xf8);
        e.call(0x00a0_1350, &args![tile, 0xfcc_u32, buffer, 1_u32]);
        let seconds = e.global::<f32>(0x0101_712c);
        let alpha = e.global::<f32>(0x011d_979c);
        let tile = e.mem.u32(this.addr() + 0xf8);
        e.call(
            0x00a0_7c60,
            &args![tile, 0xfa9_u32, 0.0_f32, alpha, seconds, 4_u32],
        );
        for index in 0x19..0x1f_u32 {
            let player = e.mem.u32(0x011d_ea3c);
            let value = e.vcall(player + 0xa4, 0xc, &args![index]).f64();
            let ratio = (value / e.global::<f64>(0x0101_7a40)) as f32;
            let format = if ratio <= 0.0 {
                0x0107_3830_u32
            } else {
                0x0107_3850_u32
            };
            let limb = e.mem.u32(0x011a_00c4 + index * 4);
            e.call(0x0040_6d00, &args![buffer, 0x104_u32, format, limb]);
            let tile_offset = 0x28 + (index + 0x1c) * 4;
            let tile = e.mem.u32(this.addr() + tile_offset);
            e.call(0x00a0_1350, &args![tile, 0xfcc_u32, buffer, 1_u32]);
            let broken = ratio == 0.0;
            let tile = e.mem.u32(this.addr() + tile_offset);
            hud_trait(e, tile, 0xff4, if broken { 2 } else { 1 });
            let seconds = e.global::<f32>(0x0101_712c);
            let alpha = e.global::<f32>(0x011d_979c);
            let tile = e.mem.u32(this.addr() + tile_offset);
            e.call(
                0x00a0_7c60,
                &args![
                    tile,
                    0xfa9_u32,
                    0.0_f32,
                    alpha,
                    seconds,
                    if broken { 3_u32 } else { 4_u32 }
                ],
            );
        }
    });
}

// Translated from 0077ee50 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::Create3DCamera` (Xbox PDB): constructs the HUD camera and
/// derives its frustum from the active renderer dimensions and settings.
// Translation incomplete: 0077ee50 - compiler SEH unwinding.
pub fn hud_main_menu_create_3d_camera(e: &mut Engine, _this: Ptr<HUDMainMenu>) -> Ptr {
    let allocation = e.call(0x00aa_13e0, &args![0x114_u32]).u32();
    let camera = if allocation == 0 {
        0
    } else {
        e.call(0x00a7_12f0, &args![allocation]).u32()
    };
    e.with_stack(16, |e, transform| {
        e.call(
            0x0041_4430,
            &args![transform, 0.0_f32, 1.0_f32, 1.0_f32, 0.0_f32],
        );
        if e.mem.u8(0x011f_9426) != 0 {
            for offset in [0, 4, 8, 12] {
                let value = e.mem.u32(0x011a_d840 + offset);
                e.mem.set_u32(transform.addr() + offset, value);
            }
        }
        e.call(0x0071_2e60, &args![camera, transform]);
    });
    e.with_stack(0x1c, |e, frustum| {
        e.call(0x00a7_1b70, &args![frustum, 0_u32]);
        let view = e.call(0x0045_c670, &args![]).u32();
        let view_angle = e.vcall(view, 0x100, &args![]).f32();
        e.mem.set_f32(frustum.addr() + 0x14, view_angle);
        let setting = e.call(0x0040_3e20, &args![0x0120_3144_u32]).u32();
        let ratio = e.mem.f32(setting);
        e.mem.set_f32(frustum.addr() + 0x10, ratio);
        let factor = (f64::from(view_angle) / f64::from(ratio)) as f32;
        e.call(0x0050_7700, &args![camera, factor]);
        let width = if e.mem.u8(0x011f_9426) != 0 {
            e.mem.i32(0x011f_9434)
        } else {
            e.call(0x004d_c200, &args![]).i32()
        };
        let height = e.call(0x004d_c1f0, &args![]).i32();
        let aspect = (f64::from(width) / f64::from(height)) as f32;
        let scale = e.global::<f32>(0x0101_6264);
        let setting = e.call(0x0040_3e20, &args![0x0120_3150_u32]).u32();
        let angle = (f64::from(e.mem.f32(setting))
            * e.global::<f64>(0x0102_3128)
            * e.global::<f64>(0x0107_0c80)) as f32;
        let left = (-(e.call(0x0057_dd70, &args![angle]).f64()) * f64::from(scale)) as f32;
        let right = (e.call(0x0057_dd70, &args![angle]).f64() * f64::from(scale)) as f32;
        let top = (-(e.call(0x0057_dd70, &args![angle]).f64())
            * f64::from(aspect)
            * f64::from(scale)) as f32;
        let bottom = (e.call(0x0057_dd70, &args![angle]).f64()
            * f64::from(aspect)
            * f64::from(scale)) as f32;
        e.mem.set_f32(frustum.addr(), left);
        e.mem.set_f32(frustum.addr() + 4, right);
        e.mem.set_f32(frustum.addr() + 8, bottom);
        e.mem.set_f32(frustum.addr() + 12, top);
        e.mem.set_u8(frustum.addr() + 0x18, 0);
        e.call(0x00a6_faf0, &args![camera, frustum]);
    });
    e.with_stack(12, |e, target| {
        let target = e
            .call(0x0041_6870, &args![target, 0.0_f32, 0.0_f32, 1.0_f32])
            .u32();
        e.with_stack(12, |e, origin| {
            let origin = e
                .call(0x0041_6870, &args![origin, 0.0_f32, 0.0_f32, 1.0_f32])
                .u32();
            e.call(0x00a7_01b0, &args![camera, origin, target]);
        });
    });
    e.call(0x0073_3fd0, &args![camera, 1.0_f32]);
    e.with_stack(12, |e, vector| {
        e.call(0x0043_d410, &args![vector, 0.0_f32, 0_u32, 0_u32]);
        e.call(0x00a5_9c60, &args![camera, vector]);
    });
    Ptr::new(camera)
}

// Translated from 0077f0d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Renders the HUD's 3D scene when its camera exists and the HUD is allowed
/// to show it, restoring scene and cancellation state afterward.
// Translation incomplete: 0077f0d0 - compiler SEH unwinding.
pub fn fn_0077f0d0(e: &mut Engine) {
    let camera = e.call(0x0055_9450, &args![0x011d_9770_u32]).u32();
    if camera == 0 {
        return;
    }
    let hud = current_hud(e);
    if hud != 0 && e.mem.u8(hud + 0x1fc) == 0 {
        return;
    }
    let renderer = e.call(0x0045_0b80, &args![1_u32]).u32();
    if e.call(0x0045_3470, &args![renderer]).u32() == 0 {
        let renderer = e.call(0x0045_0b80, &args![1_u32]).u32();
        let camera = e.call(0x0055_9450, &args![0x011d_9770_u32]).u32();
        e.vcall(renderer, 0xdc, &args![camera, 1_u32]);
    }
    // 0077f164: `0043c4b0` takes no word (plain RET, a global); the pushed 0 and 4 are
    // the two words of `007148c0` (RET 8).
    let scene = e.call(0x0043_c4b0, &args![]).u32();
    e.call(0x0071_48c0, &args![scene, 0_u32, 4_u32]);
    let face = e.call(0x0045_c670, &args![]).u32();
    let dialogue = e.call(0x008d_80e0, &args![face]).u32();
    e.call(0x00c4_f270, &args![dialogue, 1_u32]);
    let target = e.call(0x0055_9450, &args![0x011d_975c_u32]).u32();
    let exposure = e.global::<f32>(0x011a_0140);
    e.call(0x0070_9d50, &args![target, exposure]);
    let interface = e.call(0x004b_7210, &args![]).u32();
    let token = e.call(0x007f_d640, &args![interface]).u32();
    e.with_stack(4, |e, cancellation| {
        e.call(0x0063_3c90, &args![cancellation, token]);
        let active = e.call(0x0055_9450, &args![cancellation]).u32();
        e.call(0x004a_0fd0, &args![dialogue, active]);
        let scene = if target == 0 {
            e.call(0x0066_29f0, &args![face]).u32()
        } else {
            target
        };
        let renderer = e.call(0x0045_0b80, &args![1_u32]).u32();
        e.call(0x00b6_bee0, &args![scene, renderer, dialogue]);
        let token = e.call(0x0055_9450, &args![cancellation]).u32();
        e.call(0x00b6_c0d0, &args![scene, token, 0_u32]);
        e.call(0x004a_0fd0, &args![dialogue, 0_u32]);
        e.call(0x0066_b0d0, &args![cancellation, 0_u32]);
        e.call(0x00c4_f2d0, &args![dialogue]);
        e.call(0x0045_cec0, &args![cancellation]);
    });
}

// Translated from 0077f270 (decompiled, FalloutNV.exe 1.4.0.525)
/// Detaches the HUD's 3D camera sequence when both camera and active scene
/// are present, then clears the HUD's 3D visibility byte.
pub fn fn_0077f270(e: &mut Engine) {
    if current_hud(e) == 0 {
        return;
    }
    let camera = e.call(0x0055_9450, &args![0x011d_9770_u32]).u32();
    if camera == 0 {
        return;
    }
    let camera = e.call(0x0055_9450, &args![0x011d_9770_u32]).u32();
    if e.call(0x0045_bc00, &args![camera, 0_u32]).u32() == 0 {
        return;
    }
    let owner = e.call(0x0055_9450, &args![0x011d_9770_u32]).u32();
    let camera = e.call(0x0055_9450, &args![0x011d_9770_u32]).u32();
    let sequence = e.call(0x0043_b4a0, &args![camera, 0_u32]).u32();
    e.vcall(owner, 0xe8, &args![sequence]);
    let hud = current_hud(e);
    e.mem.set_u8(hud + 0x1fc, 0);
}

// Translated from 0077f2f0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Replaces the HUD camera's current model with the requested resource and
/// resets its fade control after detaching the previous sequence.
pub fn fn_0077f2f0(e: &mut Engine, resource: Ptr) {
    if current_hud(e) == 0 {
        return;
    }
    let camera = e.call(0x0055_9450, &args![0x011d_9770_u32]).u32();
    if camera == 0 {
        return;
    }
    fn_0077f270(e);
    if resource.is_null() || e.call(0x0048_cee0, &args![resource]).u32() == 0 {
        return;
    }
    let owner = e.call(0x0055_9450, &args![0x011d_9770_u32]).u32();
    let filename = e.vcall(resource.addr(), 0x14, &args![]).u32();
    let loader = e.mem.u32(0x011c_3b3c);
    let model = e
        .call(
            0x0044_7080,
            &args![loader, filename, 0_u32, 1_u32, 0_u32, 0_u32, 0_u32],
        )
        .u32();
    e.vcall(owner, 0xdc, &args![model, 1_u32]);
    let camera = e.call(0x0055_9450, &args![0x011d_9770_u32]).u32();
    e.call(0x0045_0f90, &args![camera, 1_u32]);
    e.with_stack(12, |e, vector| {
        e.call(0x0043_d410, &args![vector, 0.0_f32, 0_u32, 0_u32]);
        let camera = e.call(0x0055_9450, &args![0x011d_9770_u32]).u32();
        e.call(0x00a5_9c60, &args![camera, vector]);
    });
}

// Translated from 0077f3c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::SetGunScopeVisible` (Xbox PDB): updates the HUD scope byte
/// and selects the inverse camera fade state when its scene is available.
pub fn hud_main_menu_set_gun_scope_visible(e: &mut Engine, visible: u8) {
    if current_hud(e) == 0 {
        return;
    }
    let camera = e.call(0x0055_9450, &args![0x011d_9770_u32]).u32();
    if camera == 0 {
        return;
    }
    let camera = e.call(0x0055_9450, &args![0x011d_9770_u32]).u32();
    if e.call(0x0045_bc00, &args![camera, 0_u32]).u32() == 0 {
        return;
    }
    let hud = current_hud(e);
    e.mem.set_u8(hud + 0x1fc, visible);
    let camera = e.call(0x0055_9450, &args![0x011d_9770_u32]).u32();
    e.call(0x0045_0f90, &args![camera, u32::from(visible == 0)]);
}

// Translated from 0077f420 (decompiled, FalloutNV.exe 1.4.0.525)
/// Records the first status indicator's latest activation time and shows
/// one of its two tiles while that indicator is enabled.
pub fn fn_0077f420(e: &mut Engine, alternate: u8) {
    let now = hotkey_time(e);
    let hud = current_hud(e);
    e.mem.set_u32(hud + 0x24c, now);
    let hud = current_hud(e);
    if e.mem.u32(hud + 0x1bc) & 1 != 0 {
        let hud = current_hud(e);
        let offset = if alternate != 0 { 0x170 } else { 0x154 };
        let tile = e.mem.u32(hud + offset);
        hud_trait(e, tile, 0xfa3, 1);
    }
}

// Translated from 0077f490 (decompiled, FalloutNV.exe 1.4.0.525)
/// Records the second status indicator's latest activation time and shows
/// one of its two tiles while that indicator is enabled.
pub fn fn_0077f490(e: &mut Engine, alternate: u8) {
    let now = hotkey_time(e);
    let hud = current_hud(e);
    e.mem.set_u32(hud + 0x250, now);
    let hud = current_hud(e);
    if e.mem.u32(hud + 0x1bc) & 8 != 0 {
        let hud = current_hud(e);
        let offset = if alternate != 0 { 0x174 } else { 0x158 };
        let tile = e.mem.u32(hud + offset);
        hud_trait(e, tile, 0xfa3, 1);
    }
}

// Translated from 0077f500 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::KillQuestName` (Xbox PDB): hides name tiles and clears the
/// queued quest records unless the HUD is in the protected state five.
pub fn hud_main_menu_kill_quest_name(e: &mut Engine, _this: Ptr<HUDMainMenu>) {
    let hud = current_hud(e);
    let pool = e.mem.u32(hud + 0x60);
    let mut cursor = e.call(0x0055_9450, &args![pool + 4]).u32();
    while cursor != 0 {
        let hud = current_hud(e);
        let pool = e.mem.u32(hud + 0x60);
        let slot = reminder_advance(e, pool, &mut cursor, 0x0057_cbe0);
        let tile = e.mem.u32(slot);
        if tile != 0 {
            hud_trait(e, tile, 0xfa3, 0);
        }
    }
    let hud = current_hud(e);
    if e.mem.u32(hud + 0x1c4) != 5 {
        loop {
            let queue = e.mem.u32(0x011d_96bc);
            if fn_0077b2a0(e, Ptr::new(queue)).is_null() {
                break;
            }
            let queue = e.mem.u32(0x011d_96bc);
            let record = fn_0077b2a0(e, Ptr::new(queue));
            e.call(0x0040_1030, &args![record]);
            let queue = e.mem.u32(0x011d_96bc);
            fn_0077b2e0(e, Ptr::new(queue));
        }
        let queue = e.mem.u32(0x011d_96bc);
        fn_0077f5c0(e, Ptr::new(queue));
    }
}

// Translated from 0077f5c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Clears the fixed quest queue at 011d970c.
pub fn fn_0077f5c0(e: &mut Engine, _unused_this: Ptr) {
    e.call(0x0047_0470, &args![0x011d_970c_u32]);
}

// Translated from 0077f5e0 (decompiled, FalloutNV.exe 1.4.0.525)
/// `HUDMainMenu::KillQuestObjectives` (Xbox PDB): clears completion traits on
/// active objective tiles and drains queued objective records unless the HUD
/// is in menu mode 5.
pub fn hud_main_menu_kill_quest_objectives(e: &mut Engine, _unused_this: Ptr<HUDMainMenu>) {
    let hud = current_hud(e);
    let list = e.mem.u32(hud + 0x5c).wrapping_add(4);
    let mut cursor = e.call(0x0072_6070, &args![list]).u32();
    while cursor != 0 {
        let slot = e.with_stack(4, |e, slot| {
            e.mem.set_u32(slot.addr(), cursor);
            let hud = current_hud(e);
            let list = e.mem.u32(hud + 0x5c).wrapping_add(4);
            let element = e.call(0x0068_3520, &args![list, slot]).u32();
            cursor = e.mem.u32(slot.addr());
            element
        });
        let tile = e.mem.u32(slot);
        if tile != 0 {
            hud_trait(e, tile, 0xfa3, 0);
            hud_set_float(e, tile, 0xfa9, 0.0);
            let trait_id = e.call(0x00a0_1860, &args![0x0107_3724_u32]).u32();
            hud_set_float(e, tile, trait_id, 0.0);
            let trait_id = e.call(0x00a0_1860, &args![0x0107_3718_u32]).u32();
            hud_set_float(e, tile, trait_id, 0.0);
        }
    }

    let hud = current_hud(e);
    if e.mem.i32(hud + 0x1c4) != 5 {
        let mut queue = hud + 0x258;
        while queue != 0 && !e.call(0x0082_56d0, &args![queue]).bool() {
            let record_slot = e.call(0x0068_15c0, &args![queue]).u32();
            let record = e.mem.u32(record_slot);
            e.call(0x0040_1030, &args![record]);
            queue = e.call(0x0072_6070, &args![queue]).u32();
        }
        let hud = current_hud(e);
        e.call(0x0047_0470, &args![hud + 0x258]);
    }
}

// Translated from 0077f700 (decompiled, FalloutNV.exe 1.4.0.525)
/// Applies one frame of HUD shake using a random angle's sine and cosine,
/// lazily caching the base positions of two traits. The SEH unwind frame is
/// omitted.
// Translation incomplete: 0077f700 - SEH unwind handling is omitted.
pub fn fn_0077f700(e: &mut Engine, this: Ptr<HUDMainMenu>) {
    if e.mem.u32(0x011d_9e9c) & 1 == 0 {
        let flags = e.mem.u32(0x011d_9e9c) | 1;
        e.mem.set_u32(0x011d_9e9c, flags);
        let tile = e.mem.u32(this.addr() + 4);
        let base = e.call(0x00a0_11b0, &args![tile, 0xfa1_u32]).f32();
        e.mem.set_f32(0x011d_9e98, base);
    }
    if e.mem.u32(0x011d_9e9c) & 2 == 0 {
        let flags = e.mem.u32(0x011d_9e9c) | 2;
        e.mem.set_u32(0x011d_9e9c, flags);
        let tile = e.mem.u32(this.addr() + 4);
        let base = e.call(0x00a0_11b0, &args![tile, 0xfa2_u32]).f32();
        e.mem.set_f32(0x011d_9e94, base);
    }

    let mut x = 0.0_f32;
    let mut y = 0.0_f32;
    let shake = e.mem.f32(this.addr() + 0x274);
    let zero: f64 = e.global(0x0101_2060);
    if f64::from(shake) > zero {
        e.mem.set_u8(0x011d_9e91, 1);
        let (random_x, random_y) = e.with_stack(8, |e, output| {
            let angle = e.call(0x004a_4240, &args![]).f32();
            e.call(
                0x0041_69a0,
                &args![angle, output.addr(), output.addr().wrapping_add(4)],
            );
            (
                e.mem.f32(output.addr()),
                e.mem.f32(output.addr().wrapping_add(4)),
            )
        });
        x = random_x;
        y = random_y;
        let shake = e.mem.f32(this.addr() + 0x274);
        x = (f64::from(x) * f64::from(shake)) as f32;
        let shake = e.mem.f32(this.addr() + 0x274);
        y = (f64::from(y) * f64::from(shake)) as f32;
    }

    if e.mem.u8(0x011d_9e91) != 0 {
        let tile = e.mem.u32(this.addr() + 4);
        let value = (f64::from(e.mem.f32(0x011d_9e98)) + f64::from(x)) as f32;
        hud_set_float(e, tile, 0xfa1, value);
        let tile = e.mem.u32(this.addr() + 4);
        let value = (f64::from(e.mem.f32(0x011d_9e94)) + f64::from(y)) as f32;
        hud_set_float(e, tile, 0xfa2, value);
    }
    let shake = e.mem.f32(this.addr() + 0x274);
    let zero: f64 = e.global(0x0101_2060);
    if f64::from(shake) == zero {
        e.mem.set_u8(0x011d_9e91, 0);
    }
    e.mem.set_f32(this.addr() + 0x274, 0.0);
}

// Translated from 0077f890 (decompiled, FalloutNV.exe 1.4.0.525)
/// Adds the integer offset to the X trait of HUD tiles at +0x50, +0x54,
/// +0x4c and +0x180.
pub fn fn_0077f890(e: &mut Engine, this: Ptr<HUDMainMenu>, offset: i32) {
    for tile_offset in [0x50_u32, 0x54, 0x4c, 0x180] {
        let source = e.mem.u32(current_hud(e) + tile_offset);
        let old = e.call(0x00a0_11b0, &args![source, 0xfa1_u32]).f32();
        let value = (f64::from(old) + f64::from(offset)) as f32;
        let target = e.mem.u32(this.addr() + tile_offset);
        hud_set_float(e, target, 0xfa1, value);
    }
}

// Translated from 0077f970 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the helper's vtable and forwards the two zero constructor arguments.
pub fn fn_0077f970(e: &mut Engine, this: Ptr) -> Ptr {
    e.mem.set_u32(this.addr(), 0x0107_38cc);
    e.call(0x006b_3eb0, &args![this, 0_u32, 0_u32]);
    this
}

// Translated from 0077f9a0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Sets the array helper's vtable and clears its entries in mode 1.
pub fn fn_0077f9a0(e: &mut Engine, this: Ptr) {
    e.mem.set_u32(this.addr(), 0x0107_38cc);
    e.call(0x0084_54f0, &args![this, 1_u32]);
}

// Translated from 0077f9d0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Runs the array helper destructor and frees the object when flag bit zero is set.
pub fn fn_0077f9d0(e: &mut Engine, this: Ptr, flags: u32) -> Ptr {
    fn_0077f9a0(e, this);
    if flags & 1 != 0 {
        e.call(0x0040_1030, &args![this]);
    }
    this
}

// Translated from 0077f9c0 (decompiled, FalloutNV.exe 1.4.0.525)
/// Squares the input integer using the PC's wrapping 32-bit multiply.
pub fn fn_0077f9c0(_e: &mut Engine, value: i32) -> i32 {
    value.wrapping_mul(value)
}

/// This unit's translated functions, by exe address.
pub fn funcs() -> Vec<(u32, AbiFn)> {
    vec![
        entry!(0x0070_9430, tes_object_weap_get_ammo_regen_rate(Ptr, bool) -> f64),
        entry!(0x0076_b960, fn_0076b960(Ptr, Ptr, i32, u32, i32, i32, Ptr)),
        entry!(
            0x0076_bdc0,
            hud_main_menu_get_class(Ptr<HUDMainMenu>) -> u32
        ),
        entry!(
            0x0076_bdd0,
            hud_main_menu_attach_tile_by_id(Ptr<HUDMainMenu>, i32, Ptr)
        ),
        entry!(
            0x0076_be00,
            hud_main_menu_scalar_deleting_destructor(Ptr<HUDMainMenu>, u32) -> Ptr<HUDMainMenu>
        ),
        entry!(0x0076_be30, fn_0076be30(Ptr<HUDMainMenu>)),
        entry!(0x0076_bfe0, hud_main_menu_create() -> Ptr),
        entry!(0x0077_0070, fn_00770070(Ptr<HUDMainMenu>) -> bool),
        entry!(0x0077_00b0, fn_007700b0(Ptr<HUDMainMenu>, Ptr)),
        entry!(0x0077_00d0, fn_007700d0(Ptr<HUDMainMenu>, u8)),
        entry!(0x0077_00f0, fn_007700f0(Ptr, Ptr) -> f64),
        entry!(
            0x0077_01e0,
            hud_main_menu_update_enemy_health(Ptr<HUDMainMenu>, Ptr)
        ),
        entry!(0x0077_0430, hud_main_menu_do_idle(Ptr<HUDMainMenu>)),
        entry!(0x0077_1520, fn_00771520(Ptr)),
        entry!(
            0x0077_1540,
            hud_main_menu_update_actor_value(Ptr<HUDMainMenu>, u32)
        ),
        entry!(0x0077_1700, hud_main_menu_set_menu_mode(i32)),
        entry!(0x0077_21a0, fn_007721a0(Ptr) -> Ptr),
        entry!(0x0077_21c0, fn_007721c0(Ptr<HUDMainMenu>)),
        entry!(0x0077_2820, fn_00772820(Ptr) -> Ptr),
        entry!(0x0077_2840, hud_main_menu_heading_between_locations(Ptr, Ptr) -> f64),
        entry!(0x0077_28f0, hud_main_menu_set_region_text(Ptr)),
        entry!(0x0077_2aa0, fn_00772aa0(Ptr, Ptr)),
        entry!(0x0077_2c30, hud_main_menu_set_worldspace_text(Ptr)),
        entry!(0x0077_2d10, hud_main_menu_set_target_type(Ptr, bool)),
        entry!(0x0077_3050, fn_00773050()),
        entry!(0x0077_3190, hud_main_menu_set_enemy_actor(Ptr)),
        entry!(0x0077_32b0, fn_007732b0() -> Ptr),
        entry!(0x0077_32d0, hud_main_menu_update_sneak(Ptr<HUDMainMenu>)),
        entry!(0x0077_35b0, fn_007735b0(Ptr) -> u8),
        entry!(
            0x0077_35d0,
            hud_main_menu_is_quest_name_showing(Ptr<HUDMainMenu>) -> bool
        ),
        entry!(
            0x0077_3650,
            hud_main_menu_are_objectives_showing(Ptr<HUDMainMenu>) -> bool
        ),
        entry!(0x0077_36d0, hud_main_menu_display_quest_reminder(Ptr)),
        entry!(0x0077_37a0, fn_007737a0(Ptr, u32) -> bool),
        entry!(
            0x0077_37c0,
            hud_main_menu_update_radiation_meter_conditions(Ptr<HUDMainMenu>, f32, f32)
        ),
        entry!(0x0077_38c0, hud_main_menu_set_radiation_level(f32, f32)),
        entry!(
            0x0077_39f0,
            hud_main_menu_update_status_mode_ui(Ptr<HUDMainMenu>, bool)
        ),
        entry!(0x0077_3e90, fn_00773e90(Ptr<HUDMainMenu>, Ptr, f32, bool)),
        entry!(
            0x0077_4090,
            hud_main_menu_update_radiation_meter(Ptr<HUDMainMenu>)
        ),
        entry!(0x0077_4800, hud_main_menu_set_brightness(Ptr, u32)),
        entry!(
            0x0077_48b0,
            hud_main_menu_update_meter(Ptr, f32, i32, i32, i32)
        ),
        entry!(
            0x0077_4b00,
            hud_main_menu_update_meter_ov2(Ptr, Ptr, f32, f32, i32, i32)
        ),
        entry!(
            0x0077_4fd0,
            hud_main_menu_show_subtitle(
                Ptr<HUDMainMenu>,
                Ptr,
                u32,
                u32,
                u32,
                u32,
                u32,
                u32,
                Ptr,
                u8,
            ) -> bool
        ),
        entry!(0x0077_5210, fn_00775210(Ptr) -> Ptr),
        entry!(0x0077_5230, fn_00775230(Ptr<HUDMainMenu>) -> Ptr),
        entry!(
            0x0077_5380,
            fn_00775380(Ptr<HUDMainMenu>, Ptr, u32, Ptr, Ptr, f32, u8) -> bool
        ),
        entry!(0x0077_5670, hud_main_menu_hide_subtitle(Ptr<HUDMainMenu>)),
        entry!(0x0077_56e0, hud_main_menu_hide_message(Ptr<HUDMainMenu>)),
        entry!(0x0077_5920, fn_00775920(Ptr<HUDMainMenu>)),
        entry!(0x0077_5a00, hud_main_menu_set_info_for_ref(Ptr, u32, u8) -> bool),
        entry!(0x0077_8930, fn_00778930(Ptr) -> i32),
        entry!(0x0077_8950, fn_00778950(Ptr<HUDMainMenu>) -> u32),
        entry!(0x0077_8970, fn_00778970(Ptr<HUDMainMenu>) -> u32),
        entry!(
            0x0077_8990,
            hud_main_menu_display_water_radiation(i32, u32, u32)
        ),
        entry!(
            0x0077_8a90,
            hud_main_menu_display_water_health(i32, u32, u32)
        ),
        entry!(0x0077_8b10, hud_main_menu_refresh_info_for_ref()),
        entry!(0x0077_8b40, fn_00778b40(Ptr) -> bool),
        entry!(0x0077_8b80, hud_main_menu_get_compass_angle(f32, f32, f32, f32, f32, f32) -> f64),
        entry!(0x0077_8c20, blink_tile(Ptr, f32, f32)),
        entry!(
            0x0077_9070,
            hud_main_menu_update_compass(Ptr, Ptr, Ptr, Ptr)
        ),
        entry!(0x0077_a400, fn_0077a400(Ptr) -> Ptr),
        entry!(0x0077_a450, fn_0077a450(Ptr, f32, f32)),
        entry!(0x0077_a480, hud_main_menu_show_quest_name(Ptr)),
        entry!(0x0077_a590, fn_0077a590(Ptr) -> bool),
        entry!(0x0077_a5b0, hud_main_menu_show_quest_objective(Ptr, u8, u8)),
        entry!(
            0x0077_a650,
            hud_main_menu_update_quest_text(Ptr<HUDMainMenu>)
        ),
        entry!(0x0077_b2a0, fn_0077b2a0(Ptr) -> Ptr),
        entry!(0x0077_b2c0, fn_0077b2c0(Ptr) -> bool),
        entry!(0x0077_b2e0, fn_0077b2e0(Ptr)),
        entry!(0x0077_b300, fn_0077b300(Ptr, Ptr) -> i32),
        entry!(
            0x0077_b430,
            hud_main_menu_handle_reminder_fade(Ptr<HUDMainMenu>, Ptr)
        ),
        entry!(
            0x0077_c170,
            hud_main_menu_handle_cinematic_fade(Ptr<HUDMainMenu>, Ptr)
        ),
        entry!(0x0077_c440, fn_0077c440(u32)),
        entry!(0x0077_c4e0, hud_main_menu_update_xp_meter(Ptr<HUDMainMenu>)),
        entry!(0x0077_da30, hud_main_menu_clear_xp_meter_finished_moving()),
        entry!(
            0x0077_da60,
            hud_main_menu_update_hotkeys_wheel(Ptr<HUDMainMenu>)
        ),
        entry!(0x0077_e430, fn_0077e430(f32)),
        entry!(0x0077_e4f0, fn_0077e4f0(Ptr<HUDMainMenu>)),
        entry!(0x0077_e750, fn_0077e750(Ptr<HUDMainMenu>)),
        entry!(0x0077_eac0, fn_0077eac0(Ptr) -> bool),
        entry!(
            0x0077_eaf0,
            hud_main_menu_show_crippled_guy(Ptr<HUDMainMenu>)
        ),
        entry!(
            0x0077_ee50,
            hud_main_menu_create_3d_camera(Ptr<HUDMainMenu>) -> Ptr
        ),
        entry!(0x0077_f0d0, fn_0077f0d0()),
        entry!(0x0077_f270, fn_0077f270()),
        entry!(0x0077_f2f0, fn_0077f2f0(Ptr)),
        entry!(0x0077_f3c0, hud_main_menu_set_gun_scope_visible(u8)),
        entry!(0x0077_f420, fn_0077f420(u8)),
        entry!(0x0077_f490, fn_0077f490(u8)),
        entry!(0x0077_f500, hud_main_menu_kill_quest_name(Ptr<HUDMainMenu>)),
        entry!(0x0077_f5c0, fn_0077f5c0(Ptr)),
        entry!(
            0x0077_f5e0,
            hud_main_menu_kill_quest_objectives(Ptr<HUDMainMenu>)
        ),
        entry!(0x0077_f700, fn_0077f700(Ptr<HUDMainMenu>)),
        entry!(0x0077_f890, fn_0077f890(Ptr<HUDMainMenu>, i32)),
        entry!(0x0077_f970, fn_0077f970(Ptr) -> Ptr),
        entry!(0x0077_f9a0, fn_0077f9a0(Ptr)),
        entry!(0x0077_f9d0, fn_0077f9d0(Ptr, u32) -> Ptr),
        entry!(0x0077_f9c0, fn_0077f9c0(i32) -> i32),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_info_teleport_name_formats_destination_before_destroying_string() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x6000);
        let menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, menu.addr());
        e.mem.set_u32(menu.addr() + 0xa8, 0x5000);
        e.register_double(0x0040_37b0, |_, _| Ret::default());
        e.register_double(0x0043_a2e0, |_, words| {
            assert_eq!(words[0], 0x7000);
            Ret::default()
        });
        e.register_double(0x0055_9450, |_, _| Ret {
            eax: 0x7100,
            ..Ret::default()
        });
        e.register_double(0x0040_3df0, |_, words| {
            assert_eq!(words, [0x011d_3738]);
            Ret {
                eax: 0x7200,
                ..Ret::default()
            }
        });
        e.register_double(0x0040_6d00, |_, words| {
            assert_eq!(
                words[1..],
                [0x104, 0x0107_34e0, 0x011d_9c48, 0x7200, 0x7100]
            );
            Ret::default()
        });
        e.register_double(0x00a0_1350, |_, words| {
            assert_eq!(words[0..2], [0x5000, 0xfc4]);
            assert_eq!(words[3], 1);
            Ret::default()
        });
        e.register_double(0x0040_37d0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        set_info_teleport_name(&mut e, 0x7000);
        let calls = e.call_log.take().unwrap();
        assert_eq!(
            calls
                .iter()
                .map(|(address, _)| *address)
                .collect::<Vec<_>>(),
            [
                0x0040_37b0,
                0x0043_a2e0,
                0x0055_9450,
                0x0040_3df0,
                0x0040_6d00,
                0x00a0_1350,
                0x0040_37d0
            ]
        );
    }

    #[test]
    fn set_info_inventory_action_clears_mask_and_reloads_player_after_lock_callback() {
        let mut e = Engine::new();
        e.map(0x011c_a000, 0x1000);
        e.map(0x011d_2000, 0xd000);
        e.map(0x0101_e000, 0x1000);
        e.mem.set_f64(0x0101_e568, 255.0);
        e.mem.set_f32(0x011d_9774, 0.5);
        let menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, menu.addr());
        e.mem.set_u32(menu.addr() + 0x9c, 0x5000);
        e.register_double(0x0040_c0e0, |_, _| Ret::default());
        e.register_double(0x0040_48e0, |_, _| Ret::default());
        e.register_double(0x007a_f430, |_, _| Ret {
            eax: 0x7000,
            ..Ret::default()
        });
        e.register_double(0x00ec_43fb, |_, _| Ret::default());
        e.register_double(0x0040_3e20, |_, words| {
            assert_eq!(words, [0x011d_9774]);
            Ret {
                eax: 0x011d_9774,
                ..Ret::default()
            }
        });
        e.register_double(0x00a0_1860, |_, _| Ret {
            eax: 0xfa9,
            ..Ret::default()
        });
        e.register_double(0x00a0_12d0, |_, _| Ret::default());
        e.register_double(0x004b_71d0, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register_double(0x0087_7720, |_, words| {
            assert_eq!(words.len(), 1);
            Ret::default()
        });
        e.register_double(0x0070_3be0, |_, words| {
            assert_eq!(words[1..], [5, 3]);
            Ret {
                eax: 9,
                ..Ret::default()
            }
        });
        e.register_double(0x0070_0320, |_, words| {
            assert_eq!(words, [0x5000, 0xfa9, 0x12]);
            Ret::default()
        });
        e.register_double(0x0040_37d0, |_, _| Ret::default());
        let mut mask = 3;
        set_info_inventory_action(
            &mut e,
            Ptr::new(0x6000),
            0,
            4,
            0x2b,
            false,
            0,
            true,
            &mut mask,
        );
        assert_eq!(mask, 0);

        e.mem.set_u32(0x011d_ea3c, 0x9000);
        e.register_double(0x0057_b460, |_, _| Ret::default());
        e.register_double(0x0047_b250, |_, _| Ret::default());
        e.register_double(0x0050_21a0, |e, words| {
            assert_eq!(words, [0x7000]);
            e.mem.set_u32(0x011d_ea3c, 0x9004);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.register_double(0x0057_85e0, |_, words| {
            assert_eq!(words, [0x6000, 0x9004, 1]);
            Ret::default()
        });
        e.register_double(0x0040_3df0, |_, words| {
            assert_eq!(words, [0x011d_45a0]);
            Ret {
                eax: 0x7400,
                ..Ret::default()
            }
        });
        e.register_double(0x00a0_1350, |_, words| {
            assert_eq!(words, [0x5000, 0xfc4, 0x7400, 1]);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        set_info_inventory_action(
            &mut e,
            Ptr::new(0x6000),
            0,
            8,
            0x1c,
            false,
            0x7000,
            false,
            &mut mask,
        );
        assert_eq!(mask, 2);
        let calls = e.call_log.take().unwrap();
        let locked = calls
            .iter()
            .position(|(address, _)| *address == 0x0050_21a0)
            .unwrap();
        let owner = calls
            .iter()
            .position(|(address, _)| *address == 0x0057_85e0)
            .unwrap();
        assert!(locked < owner);

        // 00777846 uses signed JGE, so a negative target type still indexes
        // the preceding table word through 32-bit wrapping arithmetic.
        e.mem.set_u32(0x011d_515c, 0x011d_38dc);
        e.register_double(0x0040_3df0, |_, words| {
            assert_eq!(words, [0x011d_38dc]);
            Ret {
                eax: 0x7400,
                ..Ret::default()
            }
        });
        mask = 0;
        set_info_inventory_action(
            &mut e,
            Ptr::new(0x6000),
            0,
            u32::MAX,
            0x15,
            false,
            0,
            true,
            &mut mask,
        );
        assert_eq!(mask, 2);

        // 0077754f: MOV ECX,[0x011d517c] gives 00403df0 the Setting stored in table
        // slot 011d5160[7] for an actor 00566950 accepts, not the slot's address.
        e.mem.set_u32(0x011d_517c, 0x011d_3a00);
        e.register_double(0x0057_9690, |_, words| {
            assert_eq!(words, [0x6000]);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.register_double(0x0056_6950, |_, words| {
            assert_eq!(words, [0x6000]);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.register_double(0x0040_3df0, |_, words| {
            assert_eq!(words, [0x011d_3a00]);
            Ret {
                eax: 0x7400,
                ..Ret::default()
            }
        });
        mask = 0;
        set_info_inventory_action(
            &mut e,
            Ptr::new(0x6000),
            0,
            1,
            0x2a,
            true,
            0,
            true,
            &mut mask,
        );
        assert_eq!(mask, 2);
    }

    #[test]
    fn set_info_item_details_cover_water_and_unordered_numeric_values() {
        let mut e = Engine::new();
        e.map(0x011d_2000, 0xd000);
        e.map(0x0101_2000, 0x1000);
        e.map(0x0101_e000, 0x1000);
        e.mem.set_f64(0x0101_2060, 0.0);
        e.mem.set_f64(0x0101_e2c0, 50.0);
        e.mem.set_u32(0x011d_9c34, 1);
        e.mem.set_u32(0x011d_9c30, 0x5300);
        let menu = e.new_object::<HUDMainMenu>();
        let next_menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, menu.addr());
        for offset in (0x9c_u32..=0xc4).step_by(4).chain([0x138, 0x13c]) {
            e.mem.set_u32(menu.addr() + offset, 0x5000 + offset);
            e.mem.set_u32(next_menu.addr() + offset, 0x6000 + offset);
        }
        let reference = e.mem.alloc(0x20);
        let form = e.mem.alloc(0x20);
        let mut reference_slots = [0_u32; 0x8a];
        reference_slots[0x224 / 4] = 0x0300_1224;
        e.put_vtable(0x0300_1000, &reference_slots);
        e.mem.set_u32(reference, 0x0300_1000);
        let mut form_slots = [0_u32; 0x51];
        form_slots[0x140 / 4] = 0x0300_2140;
        e.put_vtable(0x0300_2000, &form_slots);
        e.mem.set_u32(form, 0x0300_2000);
        e.register_double(0x0300_1224, |_, _| Ret::default());
        e.register_double(0x007a_f430, move |_, _| Ret {
            eax: form,
            ..Ret::default()
        });
        e.register_double(0x004b_c550, |_, _| Ret {
            eax: 0x8abc,
            ..Ret::default()
        });
        e.register_double(0x004b_c650, |e, words| {
            assert_eq!(words[1], 0x8abc);
            e.mem.set_u32(words[0], 0x8000);
            Ret::default()
        });
        e.register_double(0x004b_c5f0, |_, _| Ret::default());
        e.register_double(0x005d_43c0, |_, _| Ret {
            eax: 0x8100,
            ..Ret::default()
        });
        e.register_double(0x005a_e3d0, |_, _| Ret::default());
        e.register_double(0x0040_1170, |_, _| Ret {
            eax: 0x15,
            ..Ret::default()
        });
        e.register_double(0x0300_2140, |_, _| Ret {
            eax: 0x8200,
            ..Ret::default()
        });
        e.register_double(0x0077_8950, |_, _| Ret {
            eax: 7,
            ..Ret::default()
        });
        e.register_double(0x0077_8970, |_, _| Ret {
            eax: 9,
            ..Ret::default()
        });
        e.register_double(0x0077_8a90, |_, words| {
            assert_eq!(words, [9, 0x23, 0x24]);
            Ret::default()
        });
        e.register_double(0x0077_8990, |_, words| {
            assert_eq!(words, [7, 0x25, 0x26]);
            Ret::default()
        });
        e.register_double(0x0057_9690, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register_double(0x0070_0320, |_, _| Ret::default());
        e.register_double(0x00a0_7530, |_, words| {
            assert_eq!(words, [0x5300, 8]);
            Ret::default()
        });
        e.register_double(0x00a0_11b0, move |e, words| {
            if words[1] == 0x100c {
                assert_eq!(words[0], 0x5000 + 0xa8);
                e.mem.set_u32(0x011d_96c0, next_menu.addr());
            } else if words[1] == 0xfba || words[1] == 0xfb9 {
                assert_eq!(words[0], 0x6000 + 0xa8);
            }
            Ret {
                st0: 20.0,
                ..Ret::default()
            }
        });
        e.register_double(0x00ec_62c0, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register_double(0x00a0_11f0, |_, words| {
            assert_eq!(words, [0x6000 + 0xa8, 0xfc4]);
            Ret {
                eax: 0x8300,
                ..Ret::default()
            }
        });
        e.register_double(0x005b_d5b0, |_, _| Ret {
            eax: 0x8400,
            ..Ret::default()
        });
        e.register_double(0x00a1_b020, |e, words| {
            e.mem.set_f32(words[1] + 4, 5.0);
            Ret::default()
        });
        e.register_double(0x00a0_1860, |_, _| Ret {
            eax: 0xf00,
            ..Ret::default()
        });
        e.register_double(0x00a0_12d0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert!(set_info_item_details(
            &mut e,
            Ptr::new(reference),
            false,
            false,
            4
        ));
        let calls = e.call_log.take().unwrap();
        let health = calls
            .iter()
            .position(|(address, _)| *address == 0x0077_8a90)
            .unwrap();
        let radiation = calls
            .iter()
            .position(|(address, _)| *address == 0x0077_8990)
            .unwrap();
        assert!(health < radiation);
        assert!(calls.iter().any(|(address, words)| *address == 0x0070_0320
            && words == &args![0x6000_u32 + 0xc4_u32, 0xfa3_u32, 1_u32]));
        assert_eq!(calls.last().unwrap().0, 0x004b_c5f0);
        let temporary = calls
            .iter()
            .find(|(address, _)| *address == 0x004b_c550)
            .unwrap()
            .1[0];
        let first_destructor = calls
            .iter()
            .find(|(address, _)| *address == 0x004b_c5f0)
            .unwrap();
        assert_eq!(
            first_destructor.1.as_slice(),
            args![temporary].to_vec().as_slice()
        );

        // FCOMP followed by TEST AH,0x41 selects the zero-format for NaN as
        // well as for nonpositive values; both item value and weight do this.
        e.register_double(0x0040_1170, |_, _| Ret {
            eax: 0x29,
            ..Ret::default()
        });
        e.register_double(0x004b_d400, |_, _| Ret {
            st0: f64::NAN,
            ..Ret::default()
        });
        e.register_double(0x0044_ddc0, move |_, _| Ret {
            eax: form,
            ..Ret::default()
        });
        e.register_double(0x004d_1360, |_, _| Ret::default());
        e.register_double(0x0048_ebc0, |_, _| Ret {
            st0: f64::NAN,
            ..Ret::default()
        });
        e.register_double(0x0040_6d30, |_, words| {
            assert!(words[0] == 0x011d_9c40 || words[0] == 0x011d_9c38);
            assert_eq!(words[1..], [8, 0x0104_7008]);
            Ret::default()
        });
        e.register_double(0x0040_3df0, |_, _| Ret {
            eax: 0x8500,
            ..Ret::default()
        });
        e.register_double(0x00a0_1350, |_, _| Ret::default());
        e.register_double(0x00a0_11b0, |_, _| Ret {
            st0: 20.0,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(set_info_item_details(
            &mut e,
            Ptr::new(reference),
            false,
            false,
            4
        ));
        let calls = e.call_log.take().unwrap();
        assert_eq!(
            calls
                .iter()
                .filter(|(address, _)| *address == 0x0040_6d30)
                .count(),
            2
        );
        assert!(!calls.iter().any(|(address, _)| *address == 0x0040_6d00));
    }

    #[test]
    fn set_info_for_ref_reuses_or_clears_target_tiles_in_pc_order() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x6000);
        let menu = e.new_object::<HUDMainMenu>();
        let player = 0x8000_u32;
        e.mem.set_u32(0x011d_96c0, menu.addr());
        e.mem.set_u32(0x011d_ea3c, player);
        e.mem.set_u32(menu.addr() + 0xf4, 0x5000);
        e.mem.set_u32(menu.addr() + 0x120, 0x5001);
        for (index, offset) in [0x9c_u32, 0xa8, 0xac, 0xb0, 0xb4, 0xb8, 0xbc, 0xc0, 0xc4]
            .into_iter()
            .enumerate()
        {
            e.mem.set_u32(menu.addr() + offset, 0x5100 + index as u32);
        }
        e.register_double(0x0096_3e00, |_, _| Ret::default());
        e.register_double(0x0070_0320, |_, _| Ret::default());
        e.register_double(0x00a0_11f0, |_, words| {
            assert_eq!(words, [0x5000, 0x1004]);
            Ret {
                eax: 0x7788,
                ..Ret::default()
            }
        });
        e.register_double(0x00a0_1350, |_, _| Ret::default());
        e.register_double(0x0045_8030, |_, _| Ret {
            eax: 2,
            ..Ret::default()
        });

        e.call_log = Some(vec![]);
        assert!(!hud_main_menu_set_info_for_ref(&mut e, Ptr::new(0), 0, 0,));
        let calls = e.call_log.take().unwrap();
        assert_eq!(
            calls[0],
            (0x00a0_11f0, args![0x5000_u32, 0x1004_u32].to_vec())
        );
        assert_eq!(
            calls[1],
            (
                0x00a0_1350,
                args![0x5000_u32, 0xfcc_u32, 0x7788_u32, 1_u32].to_vec()
            )
        );
        assert_eq!(
            calls[2],
            (0x0070_0320, args![0x5000_u32, 0xff4_u32, 1_u32].to_vec())
        );
        assert_eq!(
            calls[4],
            (0x0070_0320, args![0x5001_u32, 0xfa3_u32, 1_u32].to_vec())
        );

        e.mem.set_u32(menu.addr() + 0x1b8, 0x1234);
        e.mem.set_u8(0x011d_96bb, 1);
        e.call_log = Some(vec![]);
        assert!(hud_main_menu_set_info_for_ref(&mut e, Ptr::new(0), 99, 0,));
        assert_eq!(e.mem.u32(menu.addr() + 0x1b8), 0);
        assert_eq!(e.mem.u8(0x011d_96bb), 0);
        let calls = e.call_log.take().unwrap();
        assert_eq!(
            calls[0],
            (0x0096_3e00, args![player, 0xc_u32, 0_u32].to_vec())
        );
        assert_eq!(
            calls[1],
            (0x0096_3e00, args![player, 0xb_u32, 0_u32].to_vec())
        );
        for (index, (_, words)) in calls[2..11].iter().enumerate() {
            assert_eq!(words, &args![0x5100_u32 + index as u32, 0xfa3_u32, 0_u32]);
        }
        assert_eq!(
            calls[11],
            (0x0070_0320, args![0x5000_u32, 0xff4_u32, 1_u32].to_vec())
        );
        assert_eq!(
            calls[12],
            (0x00a0_11f0, args![0x5000_u32, 0x1004_u32].to_vec())
        );
        assert_eq!(
            calls[13],
            (
                0x00a0_1350,
                args![0x5000_u32, 0xfcc_u32, 0x7788_u32, 1_u32].to_vec()
            )
        );
        assert_eq!(
            calls[15],
            (0x0070_0320, args![0x5001_u32, 0xfa3_u32, 1_u32].to_vec())
        );
    }

    #[test]
    fn set_info_for_ref_records_terminal_action_before_hiding_non_actor_target() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x6000);
        e.map(0x0101_7000, 0x1000);
        e.mem.set_f32(0x0101_7b78, 10.0);
        let menu = e.new_object::<HUDMainMenu>();
        let reference = e.mem.alloc(0x40);
        e.mem.set_u32(0x011d_96c0, menu.addr());
        e.mem.set_u32(0x011d_ea3c, 0x9000);
        e.mem.set_u32(menu.addr() + 0x1b8, 0x1234);
        let mut slots = [0_u32; 0x41];
        slots[0x100 / 4] = 0x0300_1100;
        e.put_vtable(0x0300_1000, &slots);
        e.mem.set_u32(reference, 0x0300_1000);
        e.register_double(0x0300_1100, |_, _| Ret::default());
        e.register_double(0x007a_f430, |_, _| Ret {
            eax: 0xa000,
            ..Ret::default()
        });
        e.register_double(0x0040_1170, |_, _| Ret {
            eax: 0x17,
            ..Ret::default()
        });
        e.register_double(0x0056_9160, |_, _| Ret::default());
        e.register_double(0x0050_1ae0, |_, _| Ret::default());
        e.register_double(0x0056_7790, |_, _| Ret {
            eax: 0xcafe,
            ..Ret::default()
        });
        e.register_double(0x0096_3eb0, |_, _| Ret::default());
        e.register_double(0x0097_81a0, |_, _| Ret::default());
        e.register_double(0x0047_7ba0, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register_double(0x0070_0320, |_, _| Ret::default());
        e.register_double(0x00a0_11f0, |_, _| Ret {
            eax: 0x7788,
            ..Ret::default()
        });
        e.register_double(0x00a0_1350, |_, _| Ret::default());
        e.register_double(0x0045_8030, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert!(hud_main_menu_set_info_for_ref(
            &mut e,
            Ptr::new(reference),
            0,
            0,
        ));
        assert_eq!(e.mem.u32(menu.addr() + 0x1b8), reference);
        let calls = e.call_log.take().unwrap();
        let action = calls
            .iter()
            .position(|(address, _)| *address == 0x0096_3eb0)
            .unwrap();
        let clear = calls
            .iter()
            .position(|(address, _)| *address == 0x0070_0320)
            .unwrap();
        assert!(action < clear);
        assert_eq!(
            calls[action].1,
            args![0x9000_u32, 0xb_u32, 10.0_f32, 0_u32].to_vec()
        );
        assert_eq!(
            calls[action + 1],
            (0x0097_81a0, args![0x011e_0e80_u32].to_vec())
        );

        e.mem.set_u32(menu.addr() + 0x1b8, 0);
        e.mem.set_u32(0x011d_9d00, 0);
        e.register_double(0x007a_f430, |e, _| {
            let index = e.mem.u32(0x011d_9d00);
            e.mem.set_u32(0x011d_9d00, index + 1);
            Ret {
                eax: 0xa000 + index * 0x1000,
                ..Ret::default()
            }
        });
        e.register_double(0x0047_7ba0, |_, _| Ret::default());
        e.register_double(0x0057_9280, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register_double(0x0096_7090, |_, _| Ret::default());
        e.register_double(0x0057_9690, |_, _| Ret::default());
        e.register_double(0x0056_8e50, |_, _| Ret::default());
        e.register_double(0x0050_11a0, move |_, words| {
            assert_eq!(words, [0xa000, reference]);
            Ret::default()
        });
        e.register_double(0x0096_6c60, |_, _| Ret::default());
        e.register_double(0x0057_b460, |_, _| Ret::default());
        e.register_double(0x005d_43c0, |_, _| Ret {
            eax: 0xc000,
            ..Ret::default()
        });
        e.register_double(0x0041_e310, |_, _| Ret::default());
        e.register_double(0x0041_df00, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let unfinished = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            hud_main_menu_set_info_for_ref(&mut e, Ptr::new(reference), 0, 0)
        }));
        assert!(unfinished.is_err());
        assert_eq!(e.mem.u32(0x011d_9d00), 5);
        let calls = e.call_log.take().unwrap();
        assert!(calls.iter().any(|(address, words)| {
            *address == 0x0050_11a0 && words == &args![0xa000_u32, reference]
        }));

        e.map(0x0101_e000, 0x1000);
        e.mem.set_f64(0x0101_e568, 2.0);
        e.mem.set_f32(0x011d_9774, 0.5);
        e.mem.set_u32(menu.addr() + 0xac, 0x6000);
        e.mem.set_u32(menu.addr() + 0x1b8, 0);
        e.mem.set_u32(0x011d_9d00, 0);
        e.register_double(0x0096_6c60, |_, _| Ret {
            eax: 2,
            ..Ret::default()
        });
        e.register_double(0x0057_b460, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register_double(0x0064_7b70, |_, words| {
            assert_eq!(words, [0, 0]);
            Ret {
                eax: 2,
                ..Ret::default()
            }
        });
        e.register_double(0x004a_8f20, |_, words| {
            assert_eq!(words, [2, 5]);
            Ret {
                eax: 2,
                ..Ret::default()
            }
        });
        // 00776a2c / 00776a4e: the getters take only `this`; the pushed 1 is the setter's.
        e.register_double(0x0040_3df0, |_, words| {
            assert_eq!(words, [0x011d_44d4]);
            Ret {
                eax: 0xabcd,
                ..Ret::default()
            }
        });
        e.register_double(0x0040_3e20, |_, words| {
            assert_eq!(words, [0x011d_9774]);
            Ret {
                eax: 0x011d_9774,
                ..Ret::default()
            }
        });
        e.register_double(0x00a0_12d0, |_, words| {
            assert_eq!(words, args![0x6000_u32, 0xfa9_u32, 1.0_f32, 1_u32].to_vec());
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let unfinished = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            hud_main_menu_set_info_for_ref(&mut e, Ptr::new(reference), 0, 0)
        }));
        assert!(unfinished.is_err());
        let calls = e.call_log.take().unwrap();
        assert!(calls.iter().any(|(address, words)| {
            *address == 0x0040_3df0 && words == &args![0x011d_44d4_u32]
        }));

        e.map(0x0118_4000, 0x1000);
        e.mem.set_u32(0x0118_4aa0, 0x011d_1234);
        e.mem.set_u32(menu.addr() + 0x1b8, 0);
        e.mem.set_u32(0x011d_9d00, 0);
        e.register_double(0x0057_b460, |_, _| Ret::default());
        e.register_double(0x0040_3df0, |_, words| Ret {
            eax: match words[0] {
                0x011d_1234 => 0xaaaa,
                0x011d_2454 => 0xbbbb,
                other => panic!("unexpected setting {other:08x}"),
            },
            ..Ret::default()
        });
        e.register_double(0x0040_6d00, |_, words| {
            assert_eq!(
                words,
                args![
                    0x011d_9d50_u32,
                    0x104_u32,
                    0x0107_34ec_u32,
                    0xbbbb_u32,
                    0xaaaa_u32
                ]
                .to_vec()
            );
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let unfinished = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            hud_main_menu_set_info_for_ref(&mut e, Ptr::new(reference), 0, 0)
        }));
        assert!(unfinished.is_err());
        let calls = e.call_log.take().unwrap();
        assert!(calls.iter().any(|(address, words)| {
            *address == 0x00a0_1350
                && words == &args![0x6000_u32, 0xfc4_u32, 0x011d_9d50_u32, 1_u32]
        }));
    }

    #[test]
    fn set_info_for_ref_rejects_target_without_free_marker() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x6000);
        let menu = e.new_object::<HUDMainMenu>();
        let reference = e.mem.alloc(0x40);
        let player = e.mem.alloc(0x40);
        e.mem.set_u32(0x011d_96c0, menu.addr());
        e.mem.set_u32(0x011d_ea3c, player);
        let mut slots = [0_u32; 0x7e];
        slots[0x100 / 4] = 0x0300_2100;
        slots[0x1f4 / 4] = 0x0300_2200;
        e.put_vtable(0x0300_2000, &slots);
        e.mem.set_u32(reference, 0x0300_2000);
        e.mem.set_u32(player, 0x0300_2000);
        e.register_double(0x0300_2100, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register_double(0x0300_2200, |_, words| {
            assert_eq!(words.len(), 1);
            Ret {
                eax: 0xa000,
                ..Ret::default()
            }
        });
        e.register_double(0x007a_f430, |_, _| Ret {
            eax: 0xb000,
            ..Ret::default()
        });
        e.register_double(0x0040_1170, |_, _| Ret {
            eax: 0x2a,
            ..Ret::default()
        });
        e.register_double(0x0049_97b0, |_, _| Ret::default());
        e.register_double(0x0096_3e00, |_, _| Ret::default());
        e.register_double(0x0047_7ba0, |_, _| Ret::default());
        e.register_double(0x0057_9280, |_, _| Ret {
            eax: 3,
            ..Ret::default()
        });
        e.register_double(0x0050_9890, |e, words| {
            e.mem.set_u32(0x011d_9d00, words[0]);
            e.mem.set_u32(words[0], 0xface);
            Ret::default()
        });
        e.register_double(0x0056_86b0, |e, words| {
            assert_eq!(words[4], words[5] + 4);
            assert_eq!(words[4], e.mem.u32(0x011d_9d00));
            assert_eq!(e.mem.u32(words[5]), 0);
            assert_eq!(e.mem.u32(words[4]), 0xface);
            assert_eq!([words[2], words[3], words[6]], [1, 1, 1]);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert!(!hud_main_menu_set_info_for_ref(
            &mut e,
            Ptr::new(reference),
            0,
            0,
        ));
        let calls = e.call_log.take().unwrap();
        assert_eq!(
            calls
                .iter()
                .filter(|(address, _)| *address == 0x007a_f430)
                .count(),
            2
        );
        assert_eq!(
            calls
                .iter()
                .filter(|(address, _)| *address == 0x0096_3e00)
                .count(),
            2
        );
        let last = calls.last().unwrap();
        assert_eq!(last.0, 0x0056_86b0);
        assert_eq!(last.1[..2], [reference, 0xa000_u32]);
        assert_eq!(last.1.len(), 7);
    }

    #[test]
    fn set_info_for_ref_reloads_hud_and_player_after_callbacks() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x6000);
        let old_menu = e.new_object::<HUDMainMenu>();
        let new_menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, old_menu.addr());
        e.mem.set_u32(0x011d_ea3c, 0x9000);
        e.mem.set_u32(old_menu.addr() + 0xf4, 0x1111);
        e.mem.set_u32(new_menu.addr() + 0xf4, 0x2222);
        e.mem.set_u8(0x011d_96ba, 1);
        e.mem.set_u8(0x011d_96b9, 1);
        let replacement = new_menu.addr();
        e.register_double(0x00a0_11f0, move |e, words| {
            assert_eq!(words, [0x1111, 0x1004]);
            e.mem.set_u32(0x011d_96c0, replacement);
            Ret {
                eax: 0x7788,
                ..Ret::default()
            }
        });
        e.register_double(0x00a0_1350, |_, words| {
            assert_eq!(words, [0x2222, 0xfcc, 0x7788, 1]);
            Ret::default()
        });
        e.register_double(0x0070_0320, |_, words| {
            assert_eq!(words, [0x2222, 0xff4, 2]);
            Ret::default()
        });
        assert!(!hud_main_menu_set_info_for_ref(&mut e, Ptr::new(0), 0, 0));

        let reference = e.mem.alloc(0x40);
        e.mem.set_u32(new_menu.addr() + 0x1b8, reference);
        e.mem.set_u8(0x011d_96ba, 0);
        let mut slots = [0_u32; 0x41];
        slots[0x100 / 4] = 0x0300_3100;
        e.put_vtable(0x0300_3000, &slots);
        e.mem.set_u32(reference, 0x0300_3000);
        e.register_double(0x0300_3100, |e, _| {
            e.mem.set_u32(0x011d_ea3c, 0x9001);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.register_double(0x008b_c700, move |_, words| {
            assert_eq!(words, [reference, 0x9001]);
            Ret::default()
        });
        e.register_double(0x0045_8030, |_, _| Ret::default());
        assert!(!hud_main_menu_set_info_for_ref(
            &mut e,
            Ptr::new(reference),
            0,
            0,
        ));
    }

    #[test]
    fn set_info_for_ref_hides_twelve_tiles_for_unsupported_form() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x6000);
        let menu = e.new_object::<HUDMainMenu>();
        let reference = e.mem.alloc(0x40);
        e.mem.set_u32(0x011d_96c0, menu.addr());
        e.mem.set_u32(0x011d_ea3c, 0x9000);
        e.mem.set_u32(menu.addr() + 0xf4, 0x5000);
        for (index, offset) in [
            0xa8_u32, 0x9c, 0x7c, 0x80, 0x84, 0xac, 0xb0, 0xb4, 0xb8, 0xbc, 0xc0, 0xc4,
        ]
        .into_iter()
        .enumerate()
        {
            e.mem.set_u32(menu.addr() + offset, 0x5100 + index as u32);
        }
        let mut slots = [0_u32; 0xb0];
        slots[0x100 / 4] = 0x0300_4100;
        slots[0x1f4 / 4] = 0x0300_41f4;
        slots[0x224 / 4] = 0x0300_4224;
        slots[0x22c / 4] = 0x0300_422c;
        slots[0x274 / 4] = 0x0300_4274;
        e.put_vtable(0x0300_4000, &slots);
        e.mem.set_u32(reference, 0x0300_4000);
        e.register_double(0x0300_4100, |_, _| Ret::default());
        e.register_double(0x007a_f430, |_, _| Ret {
            eax: 0xb000,
            ..Ret::default()
        });
        e.register_double(0x0040_1170, |_, _| Ret {
            eax: 0x20,
            ..Ret::default()
        });
        e.register_double(0x0056_9160, |_, _| Ret::default());
        e.register_double(0x0047_7ba0, |_, _| Ret::default());
        e.register_double(0x0057_9280, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register_double(0x0096_7090, |_, _| Ret::default());
        e.register_double(0x0057_9690, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register_double(0x0070_0320, |_, _| Ret::default());
        e.register_double(0x00a0_11f0, |_, words| {
            assert_eq!(words.len(), 2);
            Ret {
                eax: 0x7788,
                ..Ret::default()
            }
        });
        e.register_double(0x00a0_1350, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert!(hud_main_menu_set_info_for_ref(
            &mut e,
            Ptr::new(reference),
            0,
            0,
        ));
        let calls = e.call_log.take().unwrap();
        let tiles: Vec<_> = calls
            .iter()
            .filter(|(address, words)| *address == 0x0070_0320 && words[1] == 0xfa3)
            .map(|(_, words)| words.clone())
            .collect();
        assert_eq!(tiles.len(), 12);
        for (index, words) in tiles.iter().enumerate() {
            assert_eq!(words, &args![0x5100_u32 + index as u32, 0xfa3_u32, 0_u32]);
        }

        e.mem.set_u32(menu.addr() + 0x1b8, 0);
        e.register_double(0x0040_1170, |_, _| Ret {
            eax: 0x1e,
            ..Ret::default()
        });
        e.register_double(0x0046_f070, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert!(hud_main_menu_set_info_for_ref(
            &mut e,
            Ptr::new(reference),
            0,
            0,
        ));
        let calls = e.call_log.take().unwrap();
        assert_eq!(
            calls.last().unwrap(),
            &(0x0046_f070, args![0xb000_u32].to_vec())
        );
        assert!(calls
            .iter()
            .all(|(address, words)| *address != 0x0070_0320 || words[1] != 0xfa3));

        e.mem.set_u32(menu.addr() + 0x1b8, 0);
        e.register_double(0x0040_1170, |_, _| Ret {
            eax: 0x15,
            ..Ret::default()
        });
        e.register_double(0x0056_8e50, |_, _| Ret::default());
        e.register_double(0x0096_6c60, |_, _| Ret::default());
        e.register_double(0x0057_b460, |_, _| Ret::default());
        e.register_double(0x005d_43c0, |_, _| Ret {
            eax: 0xc000,
            ..Ret::default()
        });
        e.register_double(0x0041_e310, |_, _| Ret::default());
        e.register_double(0x0041_df00, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        let unfinished = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            hud_main_menu_set_info_for_ref(&mut e, Ptr::new(reference), 0, 0)
        }));
        assert!(unfinished.is_err());
        let calls = e.call_log.take().unwrap();
        assert_eq!(
            calls
                .iter()
                .filter(|(address, _)| *address == 0x0096_6c60)
                .count(),
            3
        );
        assert!(calls.iter().any(|(address, words)| {
            *address == 0x0070_0320 && words == &args![0x5105_u32, 0xfa9_u32, 0_u32]
        }));
        assert_eq!(calls.last().unwrap().0, 0x0070_0320);

        e.map(0x0101_e000, 0x1000);
        e.mem.set_f64(0x0101_e568, 2.0);
        e.mem.set_f32(0x011d_9774, 0.5);
        e.mem.set_u32(menu.addr() + 0x1b8, 0);
        e.register_double(0x0041_df00, |_, _| Ret::default());
        e.register_double(0x0055_d310, |_, _| Ret {
            eax: 0x100,
            ..Ret::default()
        });
        e.register_double(0x0057_5590, |_, _| Ret::default());
        // 00776d04: `00403e20` takes only `this`; the pushed 1 is `00a012d0`'s.
        e.register_double(0x0040_3e20, |_, words| {
            assert_eq!(words, [0x011d_9774]);
            Ret {
                eax: 0x011d_9774,
                ..Ret::default()
            }
        });
        e.register_double(0x00a0_12d0, |_, words| {
            assert_eq!(words, args![0x5106_u32, 0xfa9_u32, 1.0_f32, 1_u32].to_vec());
            Ret::default()
        });
        e.call_log = Some(vec![]);
        let unfinished = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            hud_main_menu_set_info_for_ref(&mut e, Ptr::new(reference), 0, 0)
        }));
        assert!(unfinished.is_err());
        let calls = e.call_log.take().unwrap();
        assert!(calls
            .iter()
            .any(|(address, words)| { *address == 0x0055_d310 && words == &args![reference] }));
        assert_eq!(calls.last().unwrap().0, 0x00a0_12d0);

        e.map(0x9000, 0x1000);
        e.map(0x0102_f000, 0x1000);
        e.mem.set_f64(0x0102_f248, 57.295_776_367_187_5);
        let mut player_slots = [0_u32; 0xb0];
        player_slots[0x1f4 / 4] = 0x0300_51f4;
        player_slots[0x2bc / 4] = 0x0300_52bc;
        e.put_vtable(0x0300_5000, &player_slots);
        e.mem.set_u32(0x9000, 0x0300_5000);
        e.register_double(0x0300_52bc, |_, words| {
            assert_eq!(words, [0x9000, 0]);
            Ret {
                st0: 1.0,
                ..Ret::default()
            }
        });
        e.register_double(0x0300_4224, move |_, words| {
            assert_eq!(words, [reference]);
            Ret::default()
        });
        e.register_double(0x0055_d520, move |_, words| {
            assert_eq!(words, [reference]);
            Ret {
                eax: 0xd000,
                ..Ret::default()
            }
        });
        e.register_double(0x0040_6d30, |_, words| {
            assert_eq!(words, [0x011d_9c48, 0x104, 0xd000]);
            Ret::default()
        });
        e.register_double(0x0095_3c20, |_, words| {
            assert_eq!(words, [0x9000]);
            Ret::default()
        });
        e.register_double(0x0052_7080, move |_, words| {
            assert_eq!(words, [reference, 0x53]);
            Ret::default()
        });
        e.mem.set_u32(menu.addr() + 0x1b8, 0);
        e.call_log = Some(vec![]);
        let unfinished = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            hud_main_menu_set_info_for_ref(&mut e, Ptr::new(reference), 0, 0)
        }));
        assert!(unfinished.is_err());
        let calls = e.call_log.take().unwrap();
        assert_eq!(
            calls
                .iter()
                .find(|(address, _)| *address == 0x0040_6d30)
                .unwrap(),
            &(
                0x0040_6d30,
                args![0x011d_9c48_u32, 0x104_u32, 0xd000_u32].to_vec()
            )
        );
        assert!(calls.iter().any(|(address, words)| {
            *address == 0x00a0_1350
                && words == &args![0x5100_u32, 0xfc4_u32, 0x0102_0770_u32, 1_u32]
        }));

        e.mem.set_u8(0x011d_9c48, b'A');
        e.mem.set_u32(menu.addr() + 0x1b8, 0);
        e.register_double(0x0041_8770, |_, words| {
            assert_eq!(words, [0xc000]);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        let unfinished = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            hud_main_menu_set_info_for_ref(&mut e, Ptr::new(reference), 0, 0)
        }));
        assert!(unfinished.is_err());
        let calls = e.call_log.take().unwrap();
        assert!(calls.iter().any(|(address, words)| {
            *address == 0x00a0_1350
                && words == &args![0x5100_u32, 0xfc4_u32, 0x011d_9c48_u32, 1_u32]
        }));
        let hidden: Vec<_> = calls
            .iter()
            .filter(|(address, words)| *address == 0x0070_0320 && words[1] == 0xfa3)
            .collect();
        assert_eq!(
            hidden[hidden.len() - 3..]
                .iter()
                .map(|(_, words)| words[0])
                .collect::<Vec<_>>(),
            [0x5102, 0x5103, 0x5104]
        );
        assert_eq!(
            calls
                .iter()
                .filter(|(address, _)| *address == 0x0057_9280)
                .count(),
            2
        );
        assert!(calls.iter().any(|(address, words)| {
            *address == 0x0052_7080 && words == &args![reference, 0x53_u32]
        }));

        e.mem.set_u32(menu.addr() + 0x1b8, 0);
        e.mem.set_u32(menu.addr() + 0x21c, reference);
        e.register_double(0x0300_41f4, move |_, words| {
            assert_eq!(words, [reference]);
            Ret {
                eax: 0xe000,
                ..Ret::default()
            }
        });
        e.register_double(0x0300_51f4, |_, words| {
            assert_eq!(words, [0x9000]);
            Ret {
                eax: 0xe100,
                ..Ret::default()
            }
        });
        e.register_double(0x0300_422c, move |_, words| {
            assert_eq!(words, [reference, 0]);
            Ret::default()
        });
        e.register_double(0x0077_2840, |_, words| {
            assert_eq!(words, [0xe100, 0xe000]);
            Ret {
                st0: 15.0,
                ..Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        let unfinished = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            hud_main_menu_set_info_for_ref(&mut e, Ptr::new(reference), 0, 0)
        }));
        assert!(unfinished.is_err());
        let calls = e.call_log.take().unwrap();
        assert_eq!(
            calls
                .iter()
                .filter(|(address, _)| *address == 0x0077_2840)
                .count(),
            1
        );

        e.register_double(0x0300_4224, move |_, words| {
            assert_eq!(words, [reference]);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.register_double(0x009d_9f00, move |_, words| {
            assert_eq!(words, [reference]);
            Ret {
                eax: 0xb000,
                ..Ret::default()
            }
        });
        e.register_double(0x0044_ddc0, |_, words| {
            assert_eq!(words, [0xb000]);
            Ret {
                eax: 0xc000,
                ..Ret::default()
            }
        });
        e.register_double(0x0048_2720, |_, words| {
            assert_eq!(words, [0xc000]);
            Ret {
                eax: 0xd000,
                ..Ret::default()
            }
        });
        e.mem.set_u8(0x011d_9c48, 0);
        e.mem.set_u32(menu.addr() + 0x21c, 0);
        e.mem.set_u32(menu.addr() + 0x1b8, 0);
        e.call_log = Some(vec![]);
        let unfinished = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            hud_main_menu_set_info_for_ref(&mut e, Ptr::new(reference), 0, 0)
        }));
        assert!(unfinished.is_err());
        let calls = e.call_log.take().unwrap();
        assert_eq!(
            calls
                .iter()
                .filter(|(address, _)| *address == 0x009d_9f00)
                .count(),
            2
        );
        assert_eq!(
            calls
                .iter()
                .find(|(address, _)| *address == 0x0040_6d30)
                .unwrap(),
            &(
                0x0040_6d30,
                args![0x011d_9c48_u32, 0x104_u32, 0xd000_u32].to_vec()
            )
        );
        assert!(calls.iter().any(|(address, words)| {
            *address == 0x00a0_1350
                && words == &args![0x5100_u32, 0xfc4_u32, 0x0102_0770_u32, 1_u32]
        }));

        e.mem.set_u32(menu.addr() + 0x1b8, 0);
        e.mem.set_u32(menu.addr() + 0x21c, reference);
        e.mem.set_u8(0x011d_96c5, 1);
        e.register_double(0x0300_4100, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register_double(0x0049_97b0, |_, _| Ret::default());
        e.register_double(0x0096_3e00, |_, _| Ret::default());
        e.register_double(0x0057_9280, |_, _| Ret {
            eax: 7,
            ..Ret::default()
        });
        e.register_double(0x0041_df00, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register_double(0x0059_5d10, |_, words| {
            assert_eq!(words, [0x9000]);
            Ret::default()
        });
        e.register_double(0x0300_4274, |_, _| {
            panic!("highlighted target must skip actor inventory gate")
        });
        e.register_double(0x0089_4900, |_, _| {
            panic!("highlighted target must skip actor inventory gate")
        });
        e.call_log = Some(vec![]);
        let unfinished = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            hud_main_menu_set_info_for_ref(&mut e, Ptr::new(reference), 0, 0)
        }));
        assert!(unfinished.is_err());
        let calls = e.call_log.take().unwrap();
        assert!(calls.iter().any(|(address, _)| *address == 0x0059_5d10));
        assert!(!calls.iter().any(|(address, _)| {
            *address == 0x0300_4274 || *address == 0x0089_4900 || *address == 0x008b_c700
        }));
    }

    #[test]
    fn ammo_regen_includes_mod_effect_only_when_requested() {
        let mut e = Engine::new();
        let weapon: Ptr = Ptr::new(e.mem.alloc(0x200));
        e.mem.set_f32(weapon.addr() + 0x1a4, 2.5);
        e.register_double(0x004b_cf60, |_, words| {
            assert_eq!(words[1..], [6, 0]);
            Ret {
                st0: 1.25,
                ..Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            tes_object_weap_get_ammo_regen_rate(&mut e, weapon, false),
            2.5
        );
        assert!(e.call_log.as_ref().unwrap().is_empty());
        assert_eq!(
            tes_object_weap_get_ammo_regen_rate(&mut e, weapon, true),
            3.75
        );
    }

    #[test]
    fn hud_class_matches_xml_menu_id() {
        let mut e = Engine::new();
        let menu = e.new_object::<HUDMainMenu>();
        assert_eq!(hud_main_menu_get_class(&mut e, menu), 0x3ec);
    }

    #[test]
    fn attach_tile_accepts_only_ids_one_through_eighty_six() {
        let mut e = Engine::new();
        let menu = e.new_object::<HUDMainMenu>();
        for id in [0, 87, -1] {
            hud_main_menu_attach_tile_by_id(&mut e, menu, id, Ptr::new(0x9999));
        }
        assert_eq!(e.mem.u32(menu.addr() + 0x28), 0);
        hud_main_menu_attach_tile_by_id(&mut e, menu, 1, Ptr::new(0x1111));
        hud_main_menu_attach_tile_by_id(&mut e, menu, 86, Ptr::new(0x8686));
        assert_eq!(e.mem.u32(tile(menu, 1)), 0x1111);
        assert_eq!(e.mem.u32(tile(menu, 86)), 0x8686);
    }

    #[test]
    fn validate_hud_requires_every_tile_slot_one_through_eighty_five() {
        let mut e = Engine::new();
        let menu = e.new_object::<HUDMainMenu>();
        for id in 1..0x56 {
            e.mem.set_u32(tile(menu, id), 1);
        }
        assert!(fn_00770070(&mut e, menu));
        e.mem.set_u32(tile(menu, 0x55), 0);
        assert!(!fn_00770070(&mut e, menu));
    }

    #[test]
    fn replace_compass_pointer_passes_embedded_slot_to_reference_setter() {
        let mut e = Engine::new();
        let menu = e.new_object::<HUDMainMenu>();
        e.register_double(0x0066_b0d0, move |_, words| {
            assert_eq!(words, &[menu.addr() + 0x64, 0x3333]);
            Ret::default()
        });
        fn_007700b0(&mut e, menu, Ptr::new(0x3333));
    }

    #[test]
    fn compass_visibility_writes_one_byte_at_menu_offset_ninety_one() {
        let mut e = Engine::new();
        let menu = e.new_object::<HUDMainMenu>();
        fn_007700d0(&mut e, menu, 0x7f);
        assert_eq!(e.mem.u8(menu.addr() + 0x91), 0x7f);
    }

    #[test]
    fn controller_attachment_converts_before_assigning() {
        let mut e = Engine::new();
        e.register_double(0x0045_7fe0, |_, words| {
            assert!(words.is_empty());
            Ret {
                eax: 0x4004,
                ..Ret::default()
            }
        });
        e.register_double(0x00aa_4ee0, |_, words| {
            assert_eq!(words, &[0x4000, 0x4004]);
            Ret::default()
        });
        fn_00771520(&mut e, Ptr::new(0x4000));
    }

    #[test]
    fn embedded_object_getter_reads_pc_e34_offset() {
        let mut e = Engine::new();
        let object: Ptr = Ptr::new(e.mem.alloc(0xe40));
        e.mem.set_u32(object.addr() + 0xe34, 0x5555);
        assert_eq!(fn_007721a0(&mut e, object).addr(), 0x5555);
    }

    #[test]
    fn inline_pointer_getter_uses_cc_offset() {
        let mut e = Engine::new();
        e.register_double(0x0055_9450, |_, words| {
            assert_eq!(words, &[0x40cc]);
            Ret {
                eax: 0x5000,
                ..Ret::default()
            }
        });
        assert_eq!(fn_00772820(&mut e, Ptr::new(0x4000)).addr(), 0x5000);
    }

    #[test]
    fn current_enemy_getter_handles_missing_hud() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        assert_eq!(fn_007732b0(&mut e).addr(), 0);
        let menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, menu.addr());
        e.mem.set_u32(menu.addr() + 0x21c, 0x7777);
        assert_eq!(fn_007732b0(&mut e).addr(), 0x7777);
    }

    #[test]
    fn player_state_getter_reads_byte_at_five_f8() {
        let mut e = Engine::new();
        let player: Ptr = Ptr::new(e.mem.alloc(0x600));
        e.mem.set_u8(player.addr() + 0x5f8, 0x7d);
        assert_eq!(fn_007735b0(&mut e, player), 0x7d);
    }

    #[test]
    fn quest_name_visibility_stops_when_cached_flag_is_clear() {
        let mut e = Engine::new();
        let menu = e.new_object::<HUDMainMenu>();
        assert!(!hud_main_menu_is_quest_name_showing(&mut e, menu));
    }

    #[test]
    fn objective_visibility_stops_when_cached_flag_is_clear() {
        let mut e = Engine::new();
        let menu = e.new_object::<HUDMainMenu>();
        assert!(!hud_main_menu_are_objectives_showing(&mut e, menu));
    }

    #[test]
    fn object_flag_predicate_uses_requested_mask() {
        let mut e = Engine::new();
        let object: Ptr = Ptr::new(e.mem.alloc(0x24));
        e.mem.set_u32(object.addr() + 0x20, 0b1010);
        assert!(fn_007737a0(&mut e, object, 0b1000));
        assert!(!fn_007737a0(&mut e, object, 0b0100));
    }

    #[test]
    fn brightness_role_four_writes_both_values_in_native_order() {
        let mut e = Engine::new();
        e.register_double(0x0070_0320, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        hud_main_menu_set_brightness(&mut e, Ptr::new(0x4000), 4);
        let writes: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .map(|(_, words)| words.clone())
            .collect();
        assert_eq!(
            writes,
            vec![
                args![0x4000_u32, 0xff5_u32, 0x2c_u32].to_vec(),
                args![0x4000_u32, 0xff5_u32, 100_u32].to_vec()
            ]
        );
    }

    #[test]
    fn actor_value_ignores_unrecognized_ids_before_virtual_calls() {
        let mut e = Engine::new();
        e.map(0x011d_e000, 0x1000);
        let menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_ea3c, 0x1234);
        e.call_log = Some(vec![]);
        hud_main_menu_update_actor_value(&mut e, menu, 0xff);
        assert!(e.call_log.as_ref().unwrap().is_empty());
    }

    #[test]
    fn hotkey_update_exits_when_player_or_process_is_missing() {
        let mut e = Engine::new();
        e.map(0x011d_e000, 0x1000);
        let menu = e.new_object::<HUDMainMenu>();
        fn_007721c0(&mut e, menu);
        e.mem.set_u32(0x011d_ea3c, 0x1234);
        e.register_double(0x008d_8520, |_, words| {
            assert_eq!(words, &[0x1234]);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        fn_007721c0(&mut e, menu);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn region_text_ignores_updates_before_hud_creation() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        let region: Ptr = Ptr::new(e.mem.alloc(0x110));
        e.mem.set_u8(region.addr() + 0x108, 1);
        e.call_log = Some(vec![]);
        hud_main_menu_set_region_text(&mut e, region);
        assert!(e.call_log.as_ref().unwrap().is_empty());
    }

    #[test]
    fn map_marker_update_ignores_updates_before_hud_creation() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        let state: Ptr = Ptr::new(e.mem.alloc(8));
        e.mem.set_u8(state.addr() + 4, 1);
        e.call_log = Some(vec![]);
        fn_00772aa0(&mut e, Ptr::new(0x2222), state);
        assert!(e.call_log.as_ref().unwrap().is_empty());
    }

    #[test]
    fn worldspace_text_initializes_hold_trait_once_for_null_text() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        e.register_double(0x00a0_1860, |_, words| {
            assert_eq!(words, &[0x0107_3474]);
            Ret {
                eax: 0xfeed,
                ..Ret::default()
            }
        });
        hud_main_menu_set_worldspace_text(&mut e, Ptr::NULL);
        assert_eq!(e.mem.u32(0x011d_9af4), 0xfeed);
        assert_eq!(e.mem.u32(0x011d_9af8) & 1, 1);
        e.call_log = Some(vec![]);
        hud_main_menu_set_worldspace_text(&mut e, Ptr::NULL);
        assert!(e.call_log.as_ref().unwrap().is_empty());
    }

    #[test]
    fn clear_hud_state_exits_when_menu_class_has_no_tile() {
        let mut e = Engine::new();
        e.register_double(0x0070_6f20, |_, _| Ret {
            eax: 0x3ec,
            ..Ret::default()
        });
        e.register_double(0x00a0_9030, |_, words| {
            assert_eq!(words, &[0x3ec]);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        fn_00773050(&mut e);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn set_enemy_actor_honors_global_block_before_touching_enemy() {
        let mut e = Engine::new();
        let blocked = e.mem.alloc(1);
        e.mem.set_u8(blocked, 1);
        e.register_double(0x0040_8d60, move |_, words| {
            assert_eq!(words, &[0x011e_0988]);
            Ret {
                eax: blocked,
                ..Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        hud_main_menu_set_enemy_actor(&mut e, Ptr::new(0xdead));
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn quest_reminder_ignores_missing_hud_or_quest() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        hud_main_menu_display_quest_reminder(&mut e, Ptr::new(0x4040));
        let menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, menu.addr());
        e.call_log = Some(vec![]);
        hud_main_menu_display_quest_reminder(&mut e, Ptr::NULL);
        assert!(e.call_log.as_ref().unwrap().is_empty());
    }

    #[test]
    fn radiation_sound_ignores_updates_before_hud_creation() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        e.call_log = Some(vec![]);
        hud_main_menu_set_radiation_level(&mut e, 1.0, 2.0);
        assert!(e.call_log.as_ref().unwrap().is_empty());
    }

    #[test]
    fn radiation_sound_passes_pending_words_to_sound_factory() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        e.map(0x0101_2000, 0x1000);
        e.mem.set_f64(0x0101_2060, 0.0);
        let menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, menu.addr());
        e.mem.set_u32(tile(menu, 0x13), 0x5130);
        e.register_double(0x00ad_8ce0, |_, _| Ret::default());
        e.register_double(0x0045_3a70, |_, words| {
            assert!(words.is_empty());
            Ret {
                eax: 0x6200,
                ..Ret::default()
            }
        });
        e.register_double(0x00ad_7550, |_, words| {
            assert_eq!(words.len(), 4);
            assert_eq!(words[0], 0x6200);
            assert_ne!(words[1], 0);
            assert_eq!(words[2..], [0x0107_3498, 0x11]);
            Ret {
                eax: 0x6300,
                ..Ret::default()
            }
        });
        e.register_double(0x0041_8900, |_, words| {
            assert_eq!(words[1], 0x6300);
            Ret::default()
        });
        e.register_double(0x0048_3710, |_, _| Ret::default());
        e.register_double(0x00ad_8830, |_, words| {
            assert_eq!(words[1], 0);
            Ret::default()
        });
        e.register_double(0x00a0_11b0, |_, _| Ret {
            st0: 1_000_000.0,
            ..Ret::default()
        });
        e.register_double(0x0082_5c00, |_, _| Ret::default());
        hud_main_menu_set_radiation_level(&mut e, 0.0, 1.0);
    }

    #[test]
    fn target_type_clears_tracking_flags_for_null_target() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        e.map(0x0101_2000, 0x1000);
        let menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, menu.addr());
        e.mem.set_u32(tile(menu, 0x3e), 0x4000);
        e.mem.set_f64(0x0101_2070, 1.0);
        e.register_double(0x00a0_11b0, |_, _| Ret::default());
        hud_main_menu_set_target_type(&mut e, Ptr::NULL, true);
        assert_eq!(e.mem.u8(0x011d_96ba), 0);
        assert_eq!(e.mem.u8(0x011d_96b9), 1);
    }

    #[test]
    fn positive_radiation_sets_expiry_sum_and_activity_marker() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        e.map(0x0107_3000, 0x1000);
        let menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, menu.addr());
        e.mem.set_u32(tile(menu, 0x13), 0x5130);
        e.mem.set_f64(0x0107_3490, 250.0);
        e.register_double(0x0082_5c00, |_, words| {
            assert_eq!(words, &[0x011f_6394]);
            Ret {
                eax: 100,
                ..Ret::default()
            }
        });
        e.register_double(0x00a0_12d0, |_, _| Ret::default());
        e.register_double(0x0070_0320, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        hud_main_menu_update_radiation_meter_conditions(&mut e, menu, 2.0, 3.0);
        let calls = e.call_log.as_ref().unwrap();
        assert!(calls.iter().any(|(address, words)| *address == 0x00a0_12d0
            && words == &args![0x5130_u32, 0x100a_u32, 350.0_f32, 1_u32]));
        assert!(calls.iter().any(|(address, words)| *address == 0x00a0_12d0
            && words == &args![0x5130_u32, 0x1009_u32, 5.0_f32, 1_u32]));
        assert!(calls.iter().any(|(address, words)| *address == 0x0070_0320
            && words == &args![0x5130_u32, 0x100b_u32, 1_u32]));
    }

    #[test]
    fn status_icon_high_tier_uses_shared_blink_alpha_without_advancing() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        let threshold = e.mem.alloc(4);
        e.mem.set_f32(threshold, 5.0);
        e.mem.set_f32(0x011d_9b04, 0.75);
        e.register_double(0x0040_3e20, move |_, words| {
            assert_eq!(words, &[0x011d_07fc]);
            Ret {
                eax: threshold,
                ..Ret::default()
            }
        });
        e.register_double(0x0070_0320, |_, _| Ret::default());
        e.register_double(0x00a0_12d0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        fn_00773e90(&mut e, Ptr::NULL, Ptr::new(0x4444), 6.0, false);
        let calls = e.call_log.as_ref().unwrap();
        assert!(calls.iter().any(|(address, words)| *address == 0x0070_0320
            && words == &args![0x4444_u32, 0xff4_u32, 2_u32]));
        assert!(calls.iter().any(|(address, words)| *address == 0x00a0_12d0
            && words == &args![0x4444_u32, 0xfa9_u32, 0.75_f32, 1_u32]));
        assert!(!calls.iter().any(|(address, _)| *address == 0x0082_5c00));
    }

    #[test]
    fn radiation_meter_skips_finished_idle_state() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        e.map(0x011a_0000, 0x1000);
        let menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, menu.addr());
        e.mem.set_u32(tile(menu, 0x13), 0x5130);
        e.mem.set_u32(0x011a_0120, 4);
        e.register_double(0x00a0_11b0, |_, _| Ret::default());
        e.register_double(0x0082_5c00, |_, _| Ret {
            eax: 100,
            ..Ret::default()
        });
        e.register_double(0x0050_98e0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        hud_main_menu_update_radiation_meter(&mut e, menu);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 3);
    }

    #[test]
    fn quest_text_update_queues_normal_priority_with_stored_fonts() {
        let mut e = Engine::new();
        let update = e.mem.alloc(0x324);
        e.register_double(0x0040_1000, move |_, words| {
            assert_eq!(words, &[0x324]);
            Ret {
                eax: update,
                ..Ret::default()
            }
        });
        e.register_double(0x0040_6d30, |_, _| Ret::default());
        e.register_double(0x0076_baa0, move |_, words| {
            assert_eq!(words, &[update]);
            Ret::default()
        });
        fn_0076b960(&mut e, Ptr::new(1), Ptr::new(2), 0, 750, 5, 6, Ptr::new(3));
        assert_eq!(e.mem.u32(update + 0x210), 750);
        assert_eq!(e.mem.u32(update + 0x214), 0);
        assert_eq!(e.mem.u32(update + 0x218), 5);
        assert_eq!(e.mem.u32(update + 0x21c), 6);
    }

    #[test]
    fn heading_between_locations_uses_three_float_output_and_degrees() {
        let mut e = Engine::new();
        e.map(0x0102_f000, 0x1000);
        e.mem.set_f64(0x0102_f248, 57.295_776_367_187_5);
        e.register_double(0x0043_9ef0, |e, words| {
            assert_eq!(words.len(), 3);
            assert_eq!(words[0], 0x2222);
            assert_eq!(words[2], 0x1111);
            e.mem.set_f32(words[1], 0.0);
            e.mem.set_f32(words[1] + 4, 1.0);
            e.mem.set_f32(words[1] + 8, 9.0);
            Ret::default()
        });
        e.register_double(0x004b_1460, |_, words| {
            assert_eq!(words, &[0.0_f32.to_bits()]);
            Ret {
                st0: 1.0,
                ..Ret::default()
            }
        });
        e.register_double(0x004b_1480, |_, words| Ret {
            st0: f64::from(f32::from_bits(words[0])),
            ..Ret::default()
        });
        let heading =
            hud_main_menu_heading_between_locations(&mut e, Ptr::new(0x1111), Ptr::new(0x2222));
        assert_eq!(heading, 57.295_776_f32 as f64);
    }

    #[test]
    fn status_mode_refreshes_five_labels_and_all_three_limb_icons() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        let menu = e.new_object::<HUDMainMenu>();
        let player = e.mem.alloc(0x200);
        let table = e.mem.alloc(0x20);
        e.mem.set_u32(player + 0xa4, table);
        e.mem.set_u32(table + 8, 0x0f00_0010);
        e.mem.set_u32(0x011d_96c0, menu.addr());
        e.mem.set_u32(0x011d_ea3c, player);
        let threshold = e.mem.alloc(4);
        e.mem.set_f32(threshold, 0.0);
        e.register_double(0x0040_3df0, |_, words| Ret {
            eax: words[0],
            ..Ret::default()
        });
        e.register_double(0x00a0_1350, |_, _| Ret::default());
        e.register_double(0x0064_6800, |_, _| Ret {
            st0: 1.0,
            ..Ret::default()
        });
        e.register_double(0x0f00_0010, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register_double(0x0040_3e20, move |_, _| Ret {
            eax: threshold,
            ..Ret::default()
        });
        e.register_double(0x0070_0320, |_, _| Ret::default());
        e.register_double(0x00a0_12d0, |_, _| Ret::default());
        // The blink of the last limb icon (00773e3e pushes 1): its clock, settings and clamp.
        e.map(0x0101_0000, 0x2_0000);
        e.map(0x011f_6000, 0x1000);
        e.register_double(0x0082_5c00, |_, _| Ret::default());
        e.register_double(0x0053_30e0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        hud_main_menu_update_status_mode_ui(&mut e, menu, true);
        let calls = e.call_log.as_ref().unwrap();
        assert_eq!(
            calls
                .iter()
                .filter(|(address, _)| *address == 0x0040_3df0)
                .count(),
            5
        );
        assert_eq!(
            calls
                .iter()
                .filter(|(address, _)| *address == 0x0064_6800)
                .count(),
            6
        );
        assert_eq!(
            calls
                .iter()
                .filter(|(address, _)| *address == 0x0f00_0010)
                .count(),
            4
        );
        // 00773cf2..00773e40: GetActorValue gets only the id after `this`.
        let ids: Vec<&[u32]> = calls
            .iter()
            .filter(|(address, _)| *address == 0x0f00_0010)
            .map(|(_, words)| &words[1..])
            .collect();
        assert_eq!(ids, [&[0x36][..], &[0x49], &[0x4b], &[0x4a]]);
        // Status mode: only the 0x4a icon advances the shared blink.
        assert_eq!(
            calls
                .iter()
                .filter(|(address, _)| *address == 0x0082_5c00)
                .count(),
            1
        );
    }

    #[test]
    fn enemy_health_reloads_hud_tile_after_name_callback() {
        let (mut e, meter) = meter_engine();
        e.map(0x011d_e000, 0x1000);
        let original_menu = e.new_object::<HUDMainMenu>();
        let replacement_menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, original_menu.addr());
        e.mem.set_u32(original_menu.addr() + 0x80, meter.addr());
        e.mem.set_u32(original_menu.addr() + 0x84, 0x3000);
        e.mem.set_u32(replacement_menu.addr() + 0x84, 0x4000);
        let enemy: Ptr = Ptr::new(e.mem.alloc(0x200));
        let values_table = e.mem.alloc(0x20);
        e.mem.set_u32(enemy.addr() + 0xa4, values_table);
        e.mem.set_u32(values_table, 0x0f00_0020);
        e.mem.set_u32(values_table + 8, 0x0f00_0021);
        e.register_double(0x0f00_0020, |_, words| {
            assert_eq!(words[1], 0x10);
            Ret {
                eax: 100,
                ..Ret::default()
            }
        });
        e.register_double(0x0f00_0021, |_, words| {
            assert_eq!(words[1], 0x10);
            Ret {
                eax: 25,
                ..Ret::default()
            }
        });
        e.register_double(0x0047_7ba0, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register_double(0x00a0_11b0, |_, _| Ret {
            st0: 1.0,
            ..Ret::default()
        });
        e.register_double(0x005e_58f0, |e, words| {
            assert_eq!(words[0..2], [0x2f, 0]);
            e.mem.set_f32(words[2], 0.0);
            Ret::default()
        });
        e.register_double(0x0055_d520, move |e, words| {
            assert_eq!(words, &[enemy.addr()]);
            e.mem.set_u32(0x011d_96c0, replacement_menu.addr());
            Ret {
                eax: 0x9090,
                ..Ret::default()
            }
        });
        e.register_double(0x00a0_1350, |_, words| {
            assert_eq!(words, &[0x4000, 0xfc4, 0x9090, 1]);
            Ret::default()
        });
        hud_main_menu_update_enemy_health(&mut e, original_menu, enemy);
        assert_eq!(e.mem.f32(0x011d_97b8), 0.25);
    }

    fn destructor_engine() -> (Engine, Ptr<HUDMainMenu>) {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        let menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, menu.addr());
        let audio = e.mem.alloc(4);
        let table = e.mem.alloc(0xec);
        e.mem.set_u32(audio, table);
        e.mem.set_u32(table + 0xe8, 0x0f00_0030);
        let empty_list_item = e.mem.alloc(4);
        e.register_double(0x0072_6070, |_, _| Ret::default());
        e.register_double(0x0045_0b80, move |_, _| Ret {
            eax: audio,
            ..Ret::default()
        });
        e.register_double(0x0055_9450, |_, _| Ret::default());
        e.register_double(0x0f00_0030, |_, _| Ret::default());
        e.register_double(0x0066_b0d0, |_, _| Ret::default());
        e.register_double(0x0068_15c0, move |_, _| Ret {
            eax: empty_list_item,
            ..Ret::default()
        });
        for address in [
            0x0046_ffb0,
            0x0070_1520,
            0x0048_3710,
            0x0084_54f0,
            0x00a1_c520,
        ] {
            e.register_double(address, |_, _| Ret::default());
        }
        (e, menu)
    }

    #[test]
    fn hud_destructor_clears_global_and_releases_owned_containers() {
        let (mut e, menu) = destructor_engine();
        e.call_log = Some(vec![]);
        fn_0076be30(&mut e, menu);
        assert_eq!(e.mem.u32(menu.addr()), 0x0107_2df4);
        assert_eq!(e.mem.u32(0x011d_96c0), 0);
        let calls = e.call_log.as_ref().unwrap();
        assert_eq!(
            calls
                .iter()
                .filter(|(address, _)| *address == 0x0046_ffb0)
                .count(),
            4
        );
        assert_eq!(calls.last().unwrap().0, 0x00a1_c520);
    }

    #[test]
    fn scalar_deleting_destructor_frees_only_when_flag_is_set() {
        let (mut e, menu) = destructor_engine();
        e.register_double(0x0040_1030, move |_, words| {
            assert_eq!(words, &[menu.addr()]);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(
            hud_main_menu_scalar_deleting_destructor(&mut e, menu, 1),
            menu
        );
        assert_eq!(e.call_log.as_ref().unwrap().last().unwrap().0, 0x0040_1030);
        let (mut retained, retained_menu) = destructor_engine();
        retained.call_log = Some(vec![]);
        assert_eq!(
            hud_main_menu_scalar_deleting_destructor(&mut retained, retained_menu, 0),
            retained_menu
        );
        assert!(!retained
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .any(|(address, _)| *address == 0x0040_1030));
    }

    #[test]
    fn sneak_update_uses_detail_label_without_starting_alert_fade() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        let menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, menu.addr());
        e.mem.set_u32(0x011d_ea3c, 0x1234);
        e.mem.set_u32(tile(menu, 0x18), 0x1818);
        e.register_double(0x0095_3c50, |e, words| {
            assert_eq!(words[0], 0x1234);
            e.mem.set_u8(words[1], 1);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.register_double(0x0070_0320, |_, _| Ret::default());
        e.register_double(0x0040_3df0, |_, words| {
            assert_eq!(words, &[0x011d_4804]);
            Ret {
                eax: 0x9090,
                ..Ret::default()
            }
        });
        e.register_double(0x00a0_1350, |_, words| {
            assert_eq!(words, &[0x1818, 0xfc4, 0x9090, 1]);
            Ret::default()
        });
        e.register_double(0x00a0_8070, |_, _| Ret::default());
        e.register_double(0x00a0_7fc0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        hud_main_menu_update_sneak(&mut e, menu);
        let calls = e.call_log.as_ref().unwrap();
        assert!(calls.iter().any(|(address, words)| *address == 0x0070_0320
            && words == &args![0x1818_u32, 0xff4_u32, 2_u32]));
        assert!(calls.iter().any(|(address, words)| *address == 0x0070_0320
            && words == &args![0x1818_u32, 0x1004_u32, 0_u32]));
        assert!(!calls.iter().any(|(address, _)| *address == 0x00a0_7c60));
    }

    #[test]
    fn idle_timeout_uses_x87_invalid_conversion_and_quiets_fade_inputs() {
        assert_eq!(fistp_i64_low(f64::INFINITY), 0);
        assert_eq!(fistp_i64_low(f64::NEG_INFINITY), 0);
        assert_eq!(fistp_i64_low(f64::NAN), 0);
        assert_eq!(fistp_i64_low(1.9), 1);
        assert_eq!(fistp_i64_low(-1.9), u32::MAX);
        assert_eq!(
            quiet_f32(f32::from_bits(0x7f80_0001)).to_bits(),
            0x7fc0_0001
        );
    }

    fn meter_engine() -> (Engine, Ptr) {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.map(0x011d_9000, 0x1000);
        e.mem.set_f64(0x0101_2060, 0.0);
        e.mem.set_f64(0x0101_2070, 1.0);
        e.mem.set_u32(0x011d_96c0, 1);
        e.mem.set_u32(0x011d_9c18, 7);
        e.mem.set_u32(0x011d_9c14, 0x101);
        e.mem.set_u32(0x011d_9c10, 0x102);
        e.mem.set_u32(0x011d_9c0c, 0x103);
        e.register_double(0x00ec_62c0, |_, words| {
            let bits = u64::from(words[0]) | (u64::from(words[1]) << 32);
            Ret {
                eax: f64::from_bits(bits).round_ties_even() as i32 as u32,
                ..Ret::default()
            }
        });
        e.register_double(0x0070_0320, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        let target = Ptr::new(e.mem.alloc(0x20));
        (e, target)
    }

    #[test]
    fn meter_nan_uses_zero_and_preserves_st0_precision() {
        let (mut e, target) = meter_engine();
        e.register_double(0x00a0_11b0, |_, words| Ret {
            st0: match words[1] {
                0x101 => 15.999_999_8,
                0x102 => 4.0,
                _ => 30.0,
            },
            ..Ret::default()
        });
        hud_main_menu_update_meter(&mut e, target, f32::NAN, 0, 0, 0);
        let changes: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == 0x0070_0320)
            .map(|(_, words)| words.clone())
            .collect();
        assert_eq!(
            changes,
            vec![
                args![target, 0xfb1_u32, 0_u32].to_vec(),
                args![target, 0xfa1_u32, 30_u32].to_vec()
            ]
        );
        let conversions: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == 0x00ec_62c0)
            .map(|(_, words)| f64::from_bits(u64::from(words[0]) | (u64::from(words[1]) << 32)))
            .collect();
        assert_eq!(conversions[0], 15.999_999_8);
    }

    #[test]
    fn meter_reloads_zero_after_image_callback() {
        let (mut e, target) = meter_engine();
        e.register_double(0x00a0_11b0, |e, words| {
            if words[1] == 0x102 {
                e.mem.set_f64(0x0101_2060, 1.0);
            }
            Ret {
                st0: match words[1] {
                    0x101 => 16.0,
                    0x102 => 4.0,
                    _ => 30.0,
                },
                ..Ret::default()
            }
        });
        hud_main_menu_update_meter(&mut e, target, 0.99, 0, 0, 0);
        let changes: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == 0x0070_0320)
            .map(|(_, words)| words.clone())
            .collect();
        assert_eq!(changes[0], args![target, 0xfb1_u32, 0_u32].to_vec());
    }

    fn meter_pair_engine() -> (Engine, Ptr, Ptr, u32) {
        let (mut e, first) = meter_engine();
        let second: Ptr = Ptr::new(e.mem.alloc(0x20));
        let child = e.mem.alloc(0x20);
        let child_storage = e.mem.alloc(4);
        e.mem.set_u32(child_storage, child);
        e.map(0x0102_3000, 0x1000);
        e.mem.set_f32(0x0102_3cd8, 255.0);
        e.mem.set_u32(0x011d_9c2c, 15);
        e.mem.set_u32(0x011d_9c28, 0x101);
        e.mem.set_u32(0x011d_9c24, 0x102);
        e.mem.set_u32(0x011d_9c20, 0x103);
        e.register_double(0x00a0_11b0, |_, words| Ret {
            st0: match words[1] {
                0x101 => 20.0,
                0x102 => 4.0,
                0x103 => 100.0,
                _ => panic!("unexpected trait"),
            },
            ..Ret::default()
        });
        e.register_double(0x004a_8f20, |_, words| Ret {
            eax: (words[0] as i32).min(words[1] as i32) as u32,
            ..Ret::default()
        });
        e.register_double(0x0073_ac90, move |_, words| {
            assert_eq!(words, &[second.addr() + 4]);
            Ret {
                eax: child_storage,
                ..Ret::default()
            }
        });
        e.register_double(0x00a0_7ed0, |_, _| Ret::default());
        e.register_double(0x00a0_7c60, |_, _| Ret::default());
        (e, first, second, child)
    }

    #[test]
    fn meter_pair_positions_both_tiles_and_center_child() {
        for (anchor, expected) in [
            (0, vec![(0xfb1, 8), (0xfa1, 100), (0xfb1, 12), (0xfa1, 108)]),
            (1, vec![(0xfb1, 8), (0xfa1, 92), (0xfb1, 4), (0xfa1, 88)]),
            (2, vec![(0xfb1, 8), (0xfa1, 96), (0xfb1, 4), (0xfa1, 92)]),
        ] {
            let (mut e, first, second, child) = meter_pair_engine();
            hud_main_menu_update_meter_ov2(&mut e, first, second, 0.4, 0.6, anchor, 0);
            let writes: Vec<_> = e
                .call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|(address, _)| *address == 0x0070_0320)
                .map(|(_, words)| (words[0], words[1], words[2]))
                .collect();
            let mut expected_writes = Vec::new();
            if anchor == 2 {
                expected_writes.extend([(child, 0xfb1, 4), (child, 0xfa1, 104)]);
            }
            expected_writes.extend(expected.into_iter().enumerate().map(
                |(index, (trait_id, value))| {
                    (
                        if index < 2 {
                            first.addr()
                        } else {
                            second.addr()
                        },
                        trait_id,
                        value,
                    )
                },
            ));
            assert_eq!(writes, expected_writes, "anchor {anchor}");
            let fades: Vec<_> = e
                .call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|(address, _)| *address == 0x00a0_7c60)
                .map(|(_, words)| words.clone())
                .collect();
            let targets = if anchor == 2 {
                vec![child, second.addr()]
            } else {
                vec![second.addr()]
            };
            assert_eq!(
                fades,
                targets
                    .into_iter()
                    .map(
                        |target| args![target, 0xfa9_u32, 0.0_f32, 255.0_f32, 1.0_f32, 1_u32]
                            .to_vec()
                    )
                    .collect::<Vec<_>>()
            );
        }
    }

    #[test]
    fn meter_pair_initializes_traits_before_null_guard_and_keeps_minimum() {
        let (mut e, first, second, _) = meter_pair_engine();
        e.mem.set_u32(0x011d_9c2c, 0);
        e.register_double(0x00a0_0940, |_, words| Ret {
            eax: words[0],
            ..Ret::default()
        });
        hud_main_menu_update_meter_ov2(&mut e, Ptr::new(0), second, 0.0, 0.1, 0, 2);
        let initializers: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == 0x00a0_0940)
            .map(|(_, words)| words.clone())
            .collect();
        assert_eq!(
            initializers,
            vec![
                args![0x0107_3424_u32, u32::MAX].to_vec(),
                args![0x0107_3418_u32, u32::MAX].to_vec(),
                args![0x0107_3430_u32, u32::MAX].to_vec(),
                args![0x0107_34b8_u32, u32::MAX].to_vec(),
            ]
        );
        assert_eq!(e.mem.u32(0x011d_9c2c), 15);

        e.mem.set_u32(0x011d_9c28, 0x101);
        e.mem.set_u32(0x011d_9c24, 0x102);
        e.mem.set_u32(0x011d_9c20, 0x103);
        e.call_log = Some(vec![]);
        hud_main_menu_update_meter_ov2(&mut e, first, second, 0.0, 0.1, 0, 2);
        let writes: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == 0x0070_0320)
            .map(|(_, words)| words.clone())
            .collect();
        assert_eq!(writes[2], args![second, 0xfb1_u32, 8_u32].to_vec());
    }

    #[test]
    fn subtitle_constructor_initializes_embedded_point() {
        let mut e = Engine::new();
        let subtitle: Ptr = Ptr::new(e.mem.alloc(0x11c));
        e.register_double(0x0068_15c0, |e, words| {
            e.mem.set_u32(words[0], 0x1234);
            Ret::default()
        });
        assert_eq!(fn_00775210(&mut e, subtitle), subtitle);
        assert_eq!(e.mem.u32(subtitle.addr() + 0x10c), 0x1234);
    }

    #[test]
    fn subtitle_gate_and_zero_sound_duration_fallback() {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.map(0x0101_7000, 0x1000);
        e.map(0x011d_2000, 0x1000);
        e.map(0x011d_8000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.mem.set_f64(0x0101_2060, 0.0);
        e.mem.set_f64(0x0101_7b70, 1000.0);
        e.mem.set_u32(0x011d_ea3c, 0x5000);
        let setting = e.mem.alloc(4);
        e.mem.set_u8(setting, 1);
        let rate = e.mem.alloc(4);
        e.mem.set_f32(rate, 0.5);
        let subtitle = e.mem.alloc(0x11c);
        let menu = e.new_object::<HUDMainMenu>();
        let text: Ptr = Ptr::new(e.mem.alloc(8));
        e.register_double(0x0048_3710, |e, words| {
            assert_eq!(e.mem.u32(words[0]), 0xfeed);
            Ret::default()
        });
        e.register_double(0x0040_8d60, move |_, words| {
            assert_eq!(words, &[0x011d_8928]);
            Ret {
                eax: setting,
                ..Ret::default()
            }
        });
        e.register_double(0x0040_1000, move |_, words| {
            assert_eq!(words, &[0x11c]);
            Ret {
                eax: subtitle,
                ..Ret::default()
            }
        });
        e.register_double(0x0068_15c0, |_, _| Ret::default());
        e.register_double(0x0040_6d30, |_, _| Ret::default());
        e.register_double(0x00ad_8b30, |_, _| Ret::default());
        e.register_double(0x00ec_6130, |_, _| Ret {
            eax: 5,
            ..Ret::default()
        });
        e.register_double(0x0040_3e20, move |_, words| {
            assert_eq!(words, &[0x011d_2178]);
            Ret {
                eax: rate,
                ..Ret::default()
            }
        });
        e.register_double(0x0082_5c00, |_, _| Ret {
            eax: 42,
            ..Ret::default()
        });
        e.register_double(0x007c_b2e0, move |e, words| {
            assert_eq!(e.mem.u32(words[1]), subtitle);
            Ret::default()
        });
        assert!(!hud_main_menu_show_subtitle(
            &mut e,
            menu,
            Ptr::new(0),
            0xfeed,
            0,
            0,
            1,
            2,
            3,
            Ptr::new(0),
            0,
        ));
        assert!(hud_main_menu_show_subtitle(
            &mut e,
            menu,
            text,
            0xfeed,
            0,
            0,
            1,
            2,
            3,
            Ptr::new(0),
            0,
        ));
        assert_eq!(e.mem.f32(subtitle + 0x108), 2.5);
        assert_eq!(e.mem.u32(subtitle + 0x104), 42);
        assert_eq!(e.mem.u32(subtitle + 0x10c), 1);
        assert_eq!(e.mem.u32(subtitle + 0x110), 2);
        assert_eq!(e.mem.u32(subtitle + 0x114), 3);
        assert_eq!(e.mem.i32(subtitle + 0x118), 1);

        e.mem.set_u8(setting, 0);
        e.call_log = Some(vec![]);
        assert!(!hud_main_menu_show_subtitle(
            &mut e,
            menu,
            text,
            0xfeed,
            0,
            0,
            1,
            2,
            3,
            Ptr::new(0),
            0,
        ));
        assert!(!e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .any(|(address, _)| *address == 0x0040_1000));
    }

    #[test]
    fn subtitle_selection_respects_priority_and_forced_sentinel() {
        let mut e = Engine::new();
        e.map(0x0101_3000, 0x1000);
        e.map(0x0101_6000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.mem.set_u32(0x0101_3d84, 0x7f80_0001);
        e.mem.set_f32(0x0101_6970, f32::MAX);
        let menu = e.new_object::<HUDMainMenu>();
        let player = e.mem.alloc(4);
        let position = e.mem.alloc(12);
        e.mem.set_u32(0x011d_ea3c, player);
        let mut slots = [0_u32; 0x7e];
        slots[0x1f4 / 4] = 0x0300_1000;
        e.put_vtable(0x0300_0000, &slots);
        e.mem.set_u32(player, 0x0300_0000);
        e.register_double(0x0300_1000, move |_, _| Ret {
            eax: position,
            ..Ret::default()
        });
        let records: Vec<u32> = (0..3).map(|_| e.mem.alloc(0x11c)).collect();
        for (record, distance, priority) in [
            (records[0], 10.0, 1),
            (records[1], 5.0, 0),
            (records[2], 7.0, i32::MAX),
        ] {
            e.mem.set_f32(record + 0x10c, distance);
            e.mem.set_i32(record + 0x118, priority);
        }
        let elements = e.mem.alloc(12);
        for (index, record) in records.iter().enumerate() {
            e.mem.set_u32(elements + index as u32 * 4, *record);
        }
        e.register_double(0x0044_ddc0, |_, _| Ret {
            eax: 3,
            ..Ret::default()
        });
        e.register_double(0x0087_7a30, move |_, words| Ret {
            eax: elements + words[1] * 4,
            ..Ret::default()
        });
        e.register_double(0x0043_9ef0, |e, words| {
            let x = e.mem.f32(words[0]);
            e.mem.set_f32(words[1], x);
            Ret {
                eax: words[1],
                ..Ret::default()
            }
        });
        e.register_double(0x004a_7290, |e, words| Ret {
            st0: f64::from(e.mem.f32(words[0])),
            ..Ret::default()
        });
        e.register_double(0x0068_5d20, |_, words| {
            assert_eq!(words, &args![f32::from_bits(0x7fc0_0001), 2_u32]);
            Ret {
                st0: 20.0,
                ..Ret::default()
            }
        });
        assert_eq!(fn_00775230(&mut e, menu).addr(), records[2]);
    }

    #[test]
    fn hud_message_uses_pc_icon_table_and_shows_first_record() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        let menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, menu.addr());
        e.mem.set_u32(menu.addr() + 0x90, 0x9000);
        e.mem.set_u32(menu.addr() + 0x8c, 0x9004);
        let text: Ptr = Ptr::new(e.mem.alloc(12));
        let record = e.mem.alloc(0x310);
        e.register_double(0x0040_1000, move |_, words| {
            assert_eq!(words, &[0x310]);
            Ret {
                eax: record,
                ..Ret::default()
            }
        });
        e.register_double(0x0040_6d30, |_, _| Ret::default());
        e.register_double(0x0082_56d0, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register_double(0x0090_5820, |e, words| {
            e.mem.set_u32(words[0], e.mem.u32(words[1]));
            Ret::default()
        });
        e.register_double(0x0068_15c0, |_, words| Ret {
            eax: words[0],
            ..Ret::default()
        });
        e.register_double(0x00a0_1350, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        assert!(!fn_00775380(
            &mut e,
            menu,
            Ptr::new(0),
            0,
            Ptr::new(0),
            Ptr::new(0),
            1.0,
            0,
        ));
        assert!(fn_00775380(
            &mut e,
            menu,
            text,
            0,
            Ptr::new(0),
            Ptr::new(0),
            1.5,
            0,
        ));
        assert_eq!(e.mem.f32(record + 0x30c), 1.5);
        let calls = e.call_log.as_ref().unwrap();
        assert!(calls.iter().any(|(address, words)| {
            *address == 0x0040_6d30 && words == &args![record + 0x104, 0x104_u32, 0x0102_08e0_u32]
        }));
        assert!(calls.iter().any(|(address, words)| {
            *address == 0x00a0_1350 && words == &args![0x9000_u32, 0xfc4_u32, record, 1_u32]
        }));

        let sound_name: Ptr = Ptr::new(e.mem.alloc(8));
        e.register_double(0x0045_3a70, |_, words| {
            assert!(words.is_empty());
            Ret {
                eax: 0x6200,
                ..Ret::default()
            }
        });
        e.register_double(0x00ad_7550, move |_, words| {
            assert_eq!(words.len(), 4);
            assert_eq!(words[0], 0x6200);
            assert_eq!(words[2], sound_name.addr());
            assert_eq!(words[3], 0x121);
            Ret {
                eax: 0x6300,
                ..Ret::default()
            }
        });
        e.register_double(0x00ad_8830, |_, words| {
            assert_eq!(words, &[0x6300, 0]);
            Ret::default()
        });
        e.register_double(0x0048_3710, |_, _| Ret::default());
        assert!(fn_00775380(
            &mut e,
            menu,
            text,
            0,
            Ptr::new(0),
            sound_name,
            f32::from_bits(0x7f80_0001),
            0,
        ));
        assert_eq!(e.mem.u32(record + 0x30c), 0x7fc0_0001);
    }

    #[test]
    fn urgent_hud_message_ends_current_timer_before_insertion() {
        let mut e = Engine::new();
        let menu = e.new_object::<HUDMainMenu>();
        let current = e.mem.alloc(0x310);
        let incoming = e.mem.alloc(0x310);
        let text: Ptr = Ptr::new(e.mem.alloc(8));
        e.mem.set_u32(menu.addr() + 0x18c, current);
        e.mem.set_u32(menu.addr() + 0x190, 0);
        e.mem.set_f32(current + 0x30c, 9.0);
        e.register_double(0x0040_1000, move |_, _| Ret {
            eax: incoming,
            ..Ret::default()
        });
        e.register_double(0x0040_6d30, |_, _| Ret::default());
        e.register_double(0x0082_56d0, |_, _| Ret::default());
        e.register_double(0x0072_6070, |e, words| Ret {
            eax: e.mem.u32(words[0] + 4),
            ..Ret::default()
        });
        e.register_double(0x0068_15c0, |_, words| Ret {
            eax: words[0],
            ..Ret::default()
        });
        e.register_double(0x0040_8b20, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register_double(0x005a_e3d0, move |e, words| {
            assert_eq!(e.mem.f32(current + 0x30c), 0.0);
            assert_eq!(e.mem.u32(words[1]), incoming);
            Ret::default()
        });
        fn_00775380(&mut e, menu, text, 3, Ptr::new(0), Ptr::new(0), 4.0, 1);
        assert_eq!(e.mem.f32(current + 0x30c), 0.0);
        assert_eq!(e.mem.f32(incoming + 0x30c), 4.0);
    }

    #[test]
    fn hide_subtitle_clears_text_only_outside_dialogue() {
        for dialogue_visible in [false, true] {
            let mut e = Engine::new();
            e.map(0x011d_9000, 0x1000);
            let menu = e.new_object::<HUDMainMenu>();
            e.mem.set_u32(0x011d_96c0, menu.addr());
            e.mem.set_u32(menu.addr() + 0x98, 0x2345);
            e.mem.set_u32(menu.addr() + 0x1a8, 0x6789);
            e.register_double(0x0070_2680, move |_, _| Ret {
                eax: u32::from(dialogue_visible),
                ..Ret::default()
            });
            e.register_double(0x00a0_1350, |_, _| Ret::default());
            e.register_double(0x0070_0320, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            hud_main_menu_hide_subtitle(&mut e, menu);
            assert_eq!(e.mem.u32(menu.addr() + 0x1a8), 0);
            let calls = e.call_log.as_ref().unwrap();
            assert_eq!(
                calls.iter().any(|(address, _)| *address == 0x00a0_1350),
                !dialogue_visible
            );
            assert!(calls.iter().any(|(address, words)| {
                *address == 0x0070_0320 && words == &args![0x2345_u32, 0xfa3_u32, 0_u32]
            }));
        }
    }

    #[test]
    fn hide_message_fades_tiles_after_last_record() {
        let mut e = Engine::new();
        e.map(0x0102_1000, 0x1000);
        e.map(0x011d_9000, 0x1000);
        e.mem.set_f32(0x0102_1f44, 0.35);
        e.mem.set_f32(0x011d_979c, 200.0);
        let menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, menu.addr());
        let record = e.mem.alloc(0x310);
        e.mem.set_u32(menu.addr() + 0x18c, record);
        e.mem.set_u32(menu.addr() + 0x194, 99);
        for (offset, tile) in [(0x90, 0x10), (0x8c, 0x11), (0x94, 0x12)] {
            e.mem.set_u32(menu.addr() + offset, tile);
        }
        e.register_double(0x0082_56d0, |e, words| Ret {
            eax: u32::from(e.mem.u32(words[0]) == 0),
            ..Ret::default()
        });
        e.register_double(0x0068_15c0, |_, words| Ret {
            eax: words[0],
            ..Ret::default()
        });
        e.register_double(0x00a0_11f0, |_, _| Ret {
            eax: 0x7777,
            ..Ret::default()
        });
        e.register_double(0x0040_8b20, |_, _| Ret::default());
        e.register_double(0x0090_5330, |e, words| {
            assert_eq!(e.mem.u32(words[1]), e.mem.u32(words[0]));
            e.mem.set_u32(words[0], 0);
            Ret::default()
        });
        e.register_double(0x0040_1030, |_, _| Ret::default());
        e.register_double(0x00a0_7c60, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        hud_main_menu_hide_message(&mut e, menu);
        assert_eq!(e.mem.u32(menu.addr() + 0x194), 0);
        let fades: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == 0x00a0_7c60)
            .map(|(_, words)| words.clone())
            .collect();
        assert_eq!(fades.len(), 3);
        assert_eq!(
            fades[0],
            args![0x10_u32, 0xfa9_u32, 200.0_f32, 0.0_f32, 0.35_f32, 0_u32].to_vec()
        );
    }

    #[test]
    fn clear_messages_frees_records_then_clears_tiles_and_subtitle() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        let menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, menu.addr());
        let record = e.mem.alloc(0x310);
        e.mem.set_u32(menu.addr() + 0x18c, record);
        e.mem.set_u32(menu.addr() + 0x190, 0);
        e.mem.set_u32(menu.addr() + 0x1a8, 0x8000);
        for (offset, tile) in [(0x94, 0x11), (0x90, 0x12), (0x8c, 0x13), (0x98, 0x14)] {
            e.mem.set_u32(menu.addr() + offset, tile);
        }
        e.register_double(0x0082_56d0, |e, words| Ret {
            eax: u32::from(e.mem.u32(words[0]) == 0),
            ..Ret::default()
        });
        e.register_double(0x0068_15c0, |_, words| Ret {
            eax: words[0],
            ..Ret::default()
        });
        e.register_double(0x0072_6070, |e, words| Ret {
            eax: e.mem.u32(words[0] + 4),
            ..Ret::default()
        });
        e.register_double(0x0040_1030, |_, _| Ret::default());
        e.register_double(0x0047_0470, |e, words| {
            e.mem.set_u32(words[0], 0);
            Ret::default()
        });
        e.register_double(0x00a0_12d0, |_, _| Ret::default());
        e.register_double(0x00a0_1350, |_, _| Ret::default());
        e.register_double(0x0070_2680, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register_double(0x0070_0320, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        fn_00775920(&mut e, menu);
        assert_eq!(e.mem.u32(menu.addr() + 0x1a8), 0);
        assert_eq!(e.mem.u32(menu.addr() + 0x18c), 0);
        let calls = e.call_log.as_ref().unwrap();
        let free_at = calls
            .iter()
            .position(|(address, words)| *address == 0x0040_1030 && words == &args![record])
            .unwrap();
        let clear_at = calls
            .iter()
            .position(|(address, _)| *address == 0x0047_0470)
            .unwrap();
        assert!(free_at < clear_at);
        assert_eq!(
            calls
                .iter()
                .filter(|(address, _)| *address == 0x00a0_12d0)
                .count(),
            3
        );
    }

    #[test]
    fn menu_mode_reloads_hud_after_subtitle_callback() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        let old_menu = e.new_object::<HUDMainMenu>();
        let current_menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, old_menu.addr());
        e.mem.set_u32(current_menu.addr() + 0x1c0, u32::MAX);
        e.mem.set_u32(current_menu.addr() + 0x144, 0x5050);
        e.mem.set_u32(current_menu.addr() + 0x114, 0x5114);
        e.register_double(0x0059_bb30, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register_double(0x005a_03f0, |_, _| Ret::default());
        e.register_double(0x0077_5670, move |e, words| {
            assert_eq!(words, &[old_menu.addr()]);
            e.mem.set_u32(0x011d_96c0, current_menu.addr());
            Ret::default()
        });
        e.register_double(0x00a0_11b0, |_, _| Ret {
            st0: 5.0,
            ..Ret::default()
        });
        e.register_double(0x00a0_12d0, |_, _| Ret::default());
        e.register_double(0x0070_0320, |_, _| Ret::default());
        e.register_double(0x00ad_8ce0, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        hud_main_menu_set_menu_mode(&mut e, 3);
        assert_eq!(e.mem.u32(current_menu.addr() + 0x1bc), 0x100);
        let calls = e.call_log.as_ref().unwrap();
        assert!(calls
            .iter()
            .any(|(addr, words)| *addr == 0x0070_0320
                && words == &args![0x5114_u32, 0xfa3_u32, 0_u32]));
        assert_eq!(
            calls
                .iter()
                .filter(|(addr, _)| *addr == 0x0070_0320)
                .count(),
            23
        );
    }

    #[test]
    fn do_idle_stops_before_updating_when_player_has_no_process() {
        let mut e = Engine::new();
        e.map(0x011d_e000, 0x1000);
        let menu = e.new_object::<HUDMainMenu>();
        hud_main_menu_do_idle(&mut e, menu);
        assert!(e.call_log.as_ref().map_or(true, Vec::is_empty));

        e.mem.set_u32(0x011d_ea3c, 0x1234);
        e.register_double(0x008d_6f30, |_, words| {
            assert_eq!(words, &[0x1234]);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        hud_main_menu_do_idle(&mut e, menu);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn do_idle_message_fades_reload_alpha_after_each_callback() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        e.map(0x0101_2000, 0x1000);
        e.map(0x0102_1000, 0x1000);
        let menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, menu.addr());
        for (offset, current_tile) in [(0x94, 0x4094), (0x8c, 0x408c), (0x90, 0x4090)] {
            e.mem.set_u32(menu.addr() + offset, current_tile);
        }
        e.mem.set_f64(0x0101_2060, 0.0);
        e.mem.set_f32(0x0102_1f44, 2.0);
        e.mem.set_f32(0x011d_979c, 1.0);
        e.register_double(0x005a_e380, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register_double(0x00a0_1230, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register_double(0x00a0_11b0, |_, _| Ret::default());
        e.register_double(0x00a0_7fc0, |_, _| Ret::default());
        e.register_double(0x00a0_7c60, |e, _| {
            let next = e.mem.f32(0x011d_979c) + 1.0;
            e.mem.set_f32(0x011d_979c, next);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        do_idle_messages(&mut e, menu);
        let fade_calls: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == 0x00a0_7c60)
            .map(|(_, words)| words.clone())
            .collect();
        assert_eq!(fade_calls.len(), 3);
        for (index, (tile, words)) in [
            (0x4094, &fade_calls[0]),
            (0x408c, &fade_calls[1]),
            (0x4090, &fade_calls[2]),
        ]
        .into_iter()
        .enumerate()
        {
            assert_eq!(
                words,
                &args![tile, 0xfa9_u32, 0.0_f32, index as f32 + 1.0, 2.0_f32, 0_u32]
            );
        }
    }

    #[test]
    fn create_unlocks_when_xml_does_not_produce_menu() {
        let mut e = Engine::new();
        e.register_double(0x004b_7210, |_, _| Ret {
            eax: 0x1010,
            ..Ret::default()
        });
        e.register_double(0x0070_6f20, |_, _| Ret {
            eax: 0x3ec,
            ..Ret::default()
        });
        e.register_double(0x00a0_9030, |_, _| Ret::default());
        e.register_double(0x00a0_1530, |_, _| Ret {
            st0: 4.0,
            ..Ret::default()
        });
        e.register_double(0x0071_3d70, |_, _| Ret {
            eax: 0x2020,
            ..Ret::default()
        });
        e.register_double(0x0045_38a0, |_, words| {
            assert_eq!(words, &[0x2020, 0]);
            Ret::default()
        });
        e.register_double(0x0058_6150, |_, words| {
            assert_eq!(words, &[0x1010]);
            Ret {
                eax: 0x3030,
                ..Ret::default()
            }
        });
        e.register_double(0x00a0_1b00, |_, words| {
            assert_eq!(words, &[0x3030, 0x0107_343c]);
            Ret {
                eax: 0x4040,
                ..Ret::default()
            }
        });
        e.register_double(0x00a0_3c90, |_, words| {
            assert_eq!(words, &[0x4040]);
            Ret::default()
        });
        e.register_double(0x0045_38c0, |_, words| {
            assert_eq!(words, &[0x2020]);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        assert_eq!(hud_main_menu_create(&mut e), Ptr::NULL);
        let calls = e.call_log.as_ref().unwrap();
        assert_eq!(
            calls
                .iter()
                .filter(|(address, _)| *address == 0x0071_3d70)
                .count(),
            2
        );
        assert_eq!(calls.last().unwrap().0, 0x0045_38c0);
    }

    #[test]
    fn create_maximum_depth_reads_and_sets_the_xml_tile() {
        // 0076fac0 / 0076fada / 0076fafd: ECX = [EBP-0x14], the tile 00a01b00 read.
        // JNP sets on the first match; JP skips unless the second compare is equal.
        let mut e = Engine::new();
        e.map(0x0107_0000, 0x1000);
        e.mem.set_f64(0x0107_04f0, 1.0);
        e.mem.set_f64(0x0107_04e8, 2.0);
        for (kind, gets, sets) in [
            (1.0, 1, true),
            (2.0, 2, true),
            (3.0, 2, false),
            (f64::NAN, 2, false),
        ] {
            e.register_double(0x00a0_11b0, move |_, words| {
                assert_eq!(words, &[0x4040, 0xfa7]);
                Ret {
                    st0: kind,
                    ..Ret::default()
                }
            });
            e.register_double(0x00a0_12d0, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            create_maximum_depth(&mut e, 0x4040, 4.0);
            let calls = e.call_log.take().unwrap();
            assert_eq!(
                calls
                    .iter()
                    .filter(|(address, _)| *address == 0x00a0_11b0)
                    .count(),
                gets
            );
            let set = calls.iter().find(|(address, _)| *address == 0x00a0_12d0);
            if sets {
                assert_eq!(
                    set.unwrap().1,
                    args![0x4040_u32, 0xfad_u32, 4.0_f32, 1_u32].to_vec()
                );
            } else {
                assert!(set.is_none());
            }
        }
    }

    #[test]
    fn create_subtitles_sit_above_the_info_box() {
        // 0076dfa0-0076e000: the subtitle tile (hud+0x13c) gets y = info box (hud+0x138)
        // y - subtitle height - 50.
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0080_0000);
        e.map(0x011d_9000, 0x1000);
        e.mem.set_f64(0x0101_1590, 2.0);
        let menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, menu.addr());
        e.register_double(0x00a0_9030, |_, _| Ret {
            eax: 0x3ec,
            ..Ret::default()
        });
        e.register_double(0x00a0_8b20, |_, words| Ret {
            eax: words[1],
            ..Ret::default()
        });
        e.register_double(0x00a1_ddb0, |_, words| Ret {
            eax: words[2],
            ..Ret::default()
        });
        e.register_double(0x00a0_11b0, |_, words| Ret {
            st0: match (words[0], words[1]) {
                (0x0107_3110, 0xfa2) => 300.0,
                (0x0107_30d4, 0xfa2) => 999.0,
                (0x0107_30d4, 0xfb0) => 20.0,
                _ => 1.0,
            },
            ..Ret::default()
        });
        for address in [
            0x00a0_12d0,
            0x0070_0320,
            0x00a0_1860,
            0x0040_3df0,
            0x00a0_1350,
            0x00ec_62c0,
            0x005b_d5b0,
        ] {
            e.register_double(address, |_, _| Ret::default());
        }
        e.register_double(0x00a1_b020, |_, words| Ret {
            eax: words[1],
            ..Ret::default()
        });
        let dimensions = HudDimensions {
            screen_width: 1280,
            screen_height: 720,
            bracket_width: 0,
            bracket_height: 0,
            original_x: 0,
            total_width: 0,
            image_width: 0,
        };
        e.call_log = Some(vec![]);
        create_info_subtitles(&mut e, menu.addr(), dimensions);
        let calls = e.call_log.take().unwrap();
        let y = calls
            .iter()
            .find(|(address, words)| {
                *address == 0x00a0_12d0 && words[0] == 0x0107_30d4 && words[1] == 0xfa2
            })
            .unwrap();
        assert_eq!(
            y.1,
            args![0x0107_30d4_u32, 0xfa2_u32, 230.0_f32, 1_u32].to_vec()
        );
        assert!(calls.iter().any(|(address, words)| {
            *address == 0x00a0_11b0 && words == &args![0x0107_3110_u32, 0xfa2_u32]
        }));
        assert!(!calls.iter().any(|(address, words)| {
            *address == 0x00a0_11b0 && words == &args![0x0107_30d4_u32, 0xfa2_u32]
        }));
    }

    #[test]
    fn damage_threshold_uses_exact_clamp_arity_and_clears_second_perk_output_late() {
        let mut e = Engine::new();
        e.map(0x011c_a000, 0x1000);
        e.mem.set_u32(0x011c_a278, 0x7000);
        let player: Ptr = Ptr::new(e.mem.alloc(0x200));
        let target: Ptr = Ptr::new(e.mem.alloc(0x200));
        let process = e.mem.alloc(0x20);
        let target_table = e.mem.alloc(0x20);
        let process_table = e.mem.alloc(0x200);
        e.mem.set_u32(target.addr() + 0xa4, target_table);
        e.mem.set_u32(target_table + 0x0c, 0x0f00_0001);
        e.mem.set_u32(process, process_table);
        e.mem.set_u32(process_table + 0x148, 0x0f00_0002);
        e.register_double(0x0f00_0001, move |_, words| {
            assert_eq!(words, &[target.addr() + 0xa4, 0x4c]);
            Ret {
                st0: 10.0,
                ..Ret::default()
            }
        });
        e.register_double(0x0f00_0002, move |_, words| {
            assert_eq!(words, &[process]);
            Ret::default()
        });
        e.register_double(0x0040_4010, |_, words| {
            assert_eq!(words.len(), 2);
            let value = if words[0] == 0 {
                f32::from_bits(words[1])
            } else {
                assert_eq!(words[1], 0.0_f32.to_bits());
                f32::from_bits(words[0])
            };
            Ret {
                st0: f64::from(value),
                ..Ret::default()
            }
        });
        e.register_double(0x008d_8520, move |_, words| {
            assert_eq!(words, &[player.addr()]);
            Ret {
                eax: process,
                ..Ret::default()
            }
        });
        e.register_double(0x005e_58f0, move |e, words| {
            assert_eq!(words.len(), 5);
            if words[0] == 0x3a {
                assert_eq!(words[1], player.addr());
                assert_eq!(words[2], 0x7000);
                assert_eq!(words[3], target.addr());
                assert_eq!(e.mem.f32(words[4]), 0.0);
                e.mem.set_f32(words[4], 2.0);
                e.mem.set_f32(words[4] + 4, 9.0);
            } else {
                assert_eq!(words[0], 0x38);
                assert_eq!(words[1], target.addr());
                assert_eq!(words[2], player.addr());
                assert_eq!(words[3], 0x7000);
                assert_eq!(e.mem.f32(words[4]), 0.0);
                e.mem.set_f32(words[4], 3.0);
            }
            Ret::default()
        });
        assert_eq!(fn_007700f0(&mut e, player, target), 11.0);
    }

    #[test]
    fn create_ddt_icons_use_distinct_offsets_and_hardcore_child_order() {
        let mut e = Engine::new();
        e.map(0x0100_0000, 0x0080_0000);
        e.map(0x011d_9000, 0x1000);
        let menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, menu.addr());
        for (address, value) in [
            (0x0102_0998, 5.0),
            (0x0102_40b8, 16.0),
            (0x0102_f070, 32.0),
            (0x0107_2f00, 165.0),
            (0x0106_c948, 150.0),
            (0x0102_0758, 10.0),
        ] {
            e.mem.set_f64(address, value);
        }
        e.register_double(0x00a0_9030, |_, _| Ret {
            eax: 0x3ec,
            ..Ret::default()
        });
        e.register_double(0x00a0_8b20, |_, words| Ret {
            eax: words[1],
            ..Ret::default()
        });
        e.register_double(0x00a0_11b0, |_, _| Ret {
            st0: 1.0,
            ..Ret::default()
        });
        e.register_double(0x00a0_12d0, |_, _| Ret::default());
        e.register_double(0x0070_0320, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        create_ddt_and_hardcore(&mut e, 75.0);
        let calls = e.call_log.as_ref().unwrap();
        let x = |tile, expected: f32| {
            let value = calls
                .iter()
                .find(|(address, words)| {
                    *address == 0x00a0_12d0 && words[0] == tile && words[1] == 0xfa1
                })
                .unwrap()
                .1[2];
            assert_eq!(f32::from_bits(value), expected);
        };
        x(0x0107_2f38, 7.0);
        x(0x0107_2f28, -30.0);
        x(0x0107_2f08, -163.0);
        x(0x0107_2eec, 151.0);
        let children: Vec<_> = calls
            .iter()
            .filter(|(address, words)| *address == 0x00a0_8b20 && words[0] == 0x0107_2eec)
            .map(|(_, words)| words[1])
            .collect();
        assert_eq!(
            children,
            vec![
                0x0106_75c0,
                0x0106_75b8,
                0x0106_37d4,
                0x0105_afd0,
                0x0107_2ee4
            ]
        );
    }

    #[test]
    fn do_idle_runs_immediate_updates_then_respects_200_ms_cadence() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.map(0x0101_2000, 0x1000);
        e.mem.set_f64(0x0101_2060, 0.0);
        let menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_ea3c, 0x5050);
        e.mem.set_u32(menu.addr() + 0x40, 0x4040);
        e.mem.set_u32(menu.addr() + 0x34, 1);
        e.mem.set_u32(menu.addr() + 0x38, 2);
        e.mem.set_u32(menu.addr() + 0x3c, 3);
        e.register_double(0x008d_6f30, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register_double(0x0045_7fe0, |_, _| Ret::default());
        e.register_double(0x00aa_4ee0, |_, _| Ret::default());
        e.register_double(0x0077_9070, |_, words| {
            assert_eq!(words, &[0x4040, 1, 2, 3]);
            Ret::default()
        });
        e.register_double(0x0077_da60, |_, _| Ret::default());
        e.register_double(0x008d_8520, |_, _| Ret::default());
        e.register_double(0x0077_e750, |_, _| Ret::default());
        e.register_double(0x00a0_11b0, |_, _| Ret::default());
        e.register_double(0x0082_5c00, |_, _| Ret {
            eax: 100,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        hud_main_menu_do_idle(&mut e, menu);
        assert_eq!(e.mem.u32(0x011d_97c8) & 1, 1);
        assert_eq!(e.mem.u32(0x011d_97c4), 100);
        let calls = e.call_log.as_ref().unwrap();
        assert_eq!(
            calls
                .iter()
                .filter(|(address, _)| *address == 0x0082_5c00)
                .count(),
            2
        );
        assert!(!calls.iter().any(|(address, _)| *address == 0x0071_5da0));
    }

    #[test]
    fn extra_list_count_sign_extends_the_short_result() {
        let mut e = Engine::new();
        e.register_double(0x005d_43c0, |_, words| {
            assert_eq!(words, &[0x1234]);
            Ret {
                eax: 0x5678,
                ..Ret::default()
            }
        });
        e.register_double(0x0041_8770, |_, words| {
            assert_eq!(words, &[0x5678]);
            Ret {
                eax: 0xfffe,
                ..Ret::default()
            }
        });
        assert_eq!(fn_00778930(&mut e, Ptr::new(0x1234)), -2);
    }

    #[test]
    fn seen_level_getter_preserves_float_bits() {
        let mut e = Engine::new();
        let hud = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(hud.addr() + 0x188, 0x7fc0_0021);
        assert_eq!(fn_00778950(&mut e, hud), 0x7fc0_0021);
    }

    #[test]
    fn next_seen_level_word_getter_uses_its_own_offset() {
        let mut e = Engine::new();
        let hud = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(hud.addr() + 0x188, 0x1111);
        e.mem.set_u32(hud.addr() + 0x18c, 0x2222);
        assert_eq!(fn_00778970(&mut e, hud), 0x2222);
    }

    #[test]
    fn water_radiation_raises_positive_fraction_to_one_and_converts_larger_amount() {
        for (raw, expected, converts) in [(3, 1, false), (8, 2, true)] {
            let mut e = Engine::new();
            e.map(0x011d_9000, 0x6000);
            e.map(0x0101_2000, 0x1000);
            e.mem.set_f64(0x0101_2060, 0.0);
            e.mem.set_f64(0x0101_2070, 1.0);
            let hud = e.new_object::<HUDMainMenu>();
            e.mem.set_u32(0x011d_96c0, hud.addr());
            e.mem.set_u32(0x011d_ea3c, 0x3333);
            e.mem.set_u32(hud.addr() + 0x28 + 2 * 4, 0x4444);
            e.mem.set_u32(hud.addr() + 0x28 + 3 * 4, 0x5555);
            e.register_double(0x008c_4330, |_, words| {
                assert_eq!(words, &[0x3333]);
                Ret {
                    st0: 0.5,
                    ..Ret::default()
                }
            });
            e.register_double(0x005e_58f0, |e, words| {
                assert_eq!(&words[..2], &[0x1a, 0x3333]);
                assert_eq!(e.mem.f32(words[2]), 1.0);
                e.mem.set_f32(words[2], 0.5);
                Ret::default()
            });
            e.register_double(0x00ec_62c0, |_, words| {
                assert_eq!(words, &[0, (2.0_f64.to_bits() >> 32) as u32]);
                Ret {
                    eax: 2,
                    ..Ret::default()
                }
            });
            e.register_double(0x00ec_623a, move |e, words| {
                assert_eq!(words[1..], [0x0102_0774, expected]);
                e.mem.set_u8(words[0], b'0' + expected as u8);
                e.mem.set_u8(words[0] + 1, 0);
                Ret::default()
            });
            e.register_double(0x0040_3df0, |_, words| {
                assert_eq!(words, &[0x011d_30c0]);
                Ret {
                    eax: 0x6666,
                    ..Ret::default()
                }
            });
            e.register_double(0x00a0_1350, |_, _| Ret::default());
            e.call_log = Some(vec![]);
            hud_main_menu_display_water_radiation(&mut e, raw, 2, 3);
            let calls = e.call_log.as_ref().unwrap();
            assert_eq!(
                calls.iter().any(|(address, _)| *address == 0x00ec_62c0),
                converts
            );
            let texts: Vec<_> = calls
                .iter()
                .filter(|(address, _)| *address == 0x00a0_1350)
                .collect();
            assert_eq!(texts.len(), 2);
            assert_eq!(&texts[0].1[..2], &[0x4444, 0xfc4]);
            assert_eq!(&texts[1].1, &[0x5555, 0xfc4, 0x6666, 1]);
        }
    }

    #[test]
    fn water_health_formats_amount_before_setting_label_tile() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        let hud = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, hud.addr());
        e.mem.set_u32(hud.addr() + 0x28 + 4 * 4, 0x4000);
        e.mem.set_u32(hud.addr() + 0x28 + 5 * 4, 0x5000);
        e.register_double(0x00ec_623a, |_, words| {
            assert_eq!(&words[1..], &[0x0102_0774, 17]);
            Ret::default()
        });
        e.register_double(0x0040_3df0, |_, words| {
            assert_eq!(words, &[0x011d_300c]);
            Ret {
                eax: 0x6000,
                ..Ret::default()
            }
        });
        e.register_double(0x00a0_1350, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        hud_main_menu_display_water_health(&mut e, 17, 4, 5);
        let calls = e.call_log.as_ref().unwrap();
        let texts: Vec<_> = calls
            .iter()
            .filter(|(address, _)| *address == 0x00a0_1350)
            .collect();
        assert_eq!(texts.len(), 2);
        assert_eq!(&texts[0].1[..2], &[0x4000, 0xfc4]);
        assert_eq!(&texts[1].1, &[0x5000, 0xfc4, 0x6000, 1]);
    }

    #[test]
    fn refresh_reference_does_nothing_without_hud() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        e.call_log = Some(vec![]);
        hud_main_menu_refresh_info_for_ref(&mut e);
        assert!(e.call_log.as_ref().unwrap().is_empty());
    }

    #[test]
    fn matching_last_reference_is_cleared() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        assert!(!fn_00778b40(&mut e, Ptr::new(0x1234)));
        let hud = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, hud.addr());
        e.mem.set_u32(hud.addr() + 0x1b8, 0x1234);
        assert!(!fn_00778b40(&mut e, Ptr::new(0x5678)));
        assert_eq!(e.mem.u32(hud.addr() + 0x1b8), 0x1234);
        assert!(fn_00778b40(&mut e, Ptr::new(0x1234)));
        assert_eq!(e.mem.u32(hud.addr() + 0x1b8), 0);
    }

    #[test]
    fn compass_angle_uses_both_value_vectors_and_wraps_either_side() {
        for (direction, expected_positive) in [(0.0, true), (4.0, true), (f64::NAN, false)] {
            let mut e = Engine::new();
            e.map(0x011d_e000, 0x1000);
            e.map(0x0101_f000, 0x1000);
            e.map(0x0102_f000, 0x1000);
            e.map(0x0107_3000, 0x1000);
            e.mem.set_f64(0x0101_ff58, -std::f64::consts::PI);
            e.mem.set_f64(0x0101_ff48, 2.0 * std::f64::consts::PI);
            e.mem.set_f64(0x0101_ff40, std::f64::consts::PI);
            e.mem.set_f64(0x0102_f248, 57.295_776);
            e.mem.set_f64(0x0107_34f8, 1.45);
            let player = e.mem.alloc(4);
            e.mem.set_u32(0x011d_ea3c, player);
            let mut slots = [0_u32; 0xb0];
            slots[0x2bc / 4] = 0x0300_1234;
            e.put_vtable(0x0300_0000, &slots);
            e.mem.set_u32(player, 0x0300_0000);
            e.register_double(0x0043_9ef0, |e, words| {
                assert_eq!(words.len(), 3);
                assert_eq!(words[2], words[0] + 12);
                assert_eq!(e.mem.f32(words[0]), 1.0);
                assert_eq!(e.mem.f32(words[0] + 4), 2.0);
                assert_eq!(e.mem.f32(words[2]), 4.0);
                assert_eq!(e.mem.f32(words[2] + 4), 5.0);
                e.mem.set_f32(words[1], -3.0);
                e.mem.set_f32(words[1] + 4, -3.0);
                e.mem.set_f32(words[1] + 8, -3.0);
                Ret {
                    eax: words[1],
                    ..Ret::default()
                }
            });
            e.register_double(0x004b_1550, move |e, words| {
                assert_eq!(e.mem.f32(words[0]), -3.0);
                Ret {
                    st0: direction,
                    ..Ret::default()
                }
            });
            let heading = if direction == 0.0 { 4.0 } else { 0.0 };
            e.register_double(0x0300_1234, move |_, words| {
                assert_eq!(words.len(), 2);
                assert_eq!(words[1], 0);
                Ret {
                    st0: heading,
                    ..Ret::default()
                }
            });
            let result = hud_main_menu_get_compass_angle(&mut e, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0);
            if expected_positive {
                assert!((189.0..191.0).contains(&result));
            } else {
                assert!(result.is_nan());
            }
        }
    }

    #[test]
    fn blink_tile_clamps_period_then_handles_cap_and_negative_alpha() {
        let mut e = Engine::new();
        e.map(0x011d_1000, 0x5000);
        e.map(0x011d_9000, 0x2000);
        e.map(0x0102_1000, 0x1000);
        e.mem.set_f64(0x0102_1928, 3.0);
        e.mem.set_f32(0x011d_979c, 1.0);
        for (address, value) in [
            (0x011d_1d54, 9),
            (0x011d_30d8, 3),
            (0x011d_4d44, 5),
            (0x011d_2bd4, 7),
            (0x011d_3c94, 8),
            (0x011d_30f0, 9),
            (0x011d_24f0, 3),
            (0x011d_46cc, 4),
            (0x011d_3924, 5),
        ] {
            e.mem.set_i32(address, value);
        }
        let tile = e.mem.alloc(0x40);
        e.mem.set_f32(tile + 0x14, 3.0);
        e.register_double(0x00a0_0940, |_, words| {
            assert_eq!(words, &[0x0107_3500, u32::MAX]);
            Ret {
                eax: 0x1200,
                ..Ret::default()
            }
        });
        e.register_double(0x0043_d4d0, |_, words| Ret {
            eax: words[0],
            ..Ret::default()
        });
        e.register_double(0x004b_3ab0, move |e, words| {
            let count = e.mem.u32(tile + 0x20);
            e.mem.set_u32(tile + 0x20, count + 1);
            if count == 0 {
                assert_eq!(f32::from_bits(words[4]), 81.0);
                assert_eq!(f32::from_bits(words[2]), 81.0);
            } else {
                assert_eq!(f32::from_bits(words[4]), 20.0);
                assert_eq!(f32::from_bits(words[2]), 25.0);
            }
            Ret {
                st0: if count == 2 { 3.0 } else { 0.5 },
                ..Ret::default()
            }
        });
        e.register_double(0x00a0_11b0, move |e, words| {
            assert_eq!(words[0], tile);
            let value = match words[1] {
                0x1200 => e.mem.f32(tile + 0x14),
                0xfa9 => e.mem.f32(tile + 0x10),
                0x100e => e.mem.f32(tile + 0x18),
                _ => panic!("unexpected tile trait"),
            };
            Ret {
                st0: f64::from(value),
                ..Ret::default()
            }
        });
        e.register_double(0x00a0_12d0, move |e, words| {
            assert_eq!(words[0], tile);
            assert_eq!(words.len(), 4);
            match words[1] {
                0xfa9 => e.mem.set_f32(tile + 0x10, f32::from_bits(words[2])),
                0x100e => e.mem.set_f32(tile + 0x18, f32::from_bits(words[2])),
                _ => panic!("unexpected tile trait"),
            }
            Ret::default()
        });
        e.register_double(0x0070_0320, move |e, words| {
            assert_eq!(&words[..2], &[tile, 0x1200]);
            e.mem.set_f32(tile + 0x14, words[2] as f32);
            Ret::default()
        });
        e.register_double(0x0084_d030, |_, _| Ret {
            st0: 1.0,
            ..Ret::default()
        });
        e.register_double(0x0082_5c00, |_, _| Ret {
            eax: 100,
            ..Ret::default()
        });
        blink_tile(&mut e, Ptr::new(tile), 100.0, 0.5);
        assert_eq!(e.mem.u32(0x011d_9e58) & 1, 1);
        assert_eq!(e.mem.u32(0x011d_9e54), 0x1200);
        assert_eq!(e.mem.f32(tile + 0x10), 0.5);
        assert_eq!(e.mem.f32(tile + 0x14), 3.0);
        e.mem.set_f32(tile + 0x10, 0.0);
        e.mem.set_f32(tile + 0x14, 1.0);
        blink_tile(&mut e, Ptr::new(tile), 20.0, 0.5);
        assert_eq!(e.mem.u32(tile + 0x20), 3);
        assert_eq!(e.mem.f32(tile + 0x10), 0.0);
        assert_eq!(e.mem.f32(tile + 0x14), 0.0);
        assert_eq!(e.mem.f32(tile + 0x18), 103.0);
    }

    #[test]
    fn compass_reloads_callback_mutated_targets_and_vats_actor() {
        let mut e = Engine::new();
        for (base, size) in [
            (0x011d_0000, 0x6000),
            (0x011d_9000, 0x6000),
            (0x011c_d000, 0x1000),
            (0x0101_2000, 0x1000),
            (0x0101_1000, 0x1000),
            (0x0101_e000, 0x1000),
            (0x0101_f000, 0x1000),
            (0x0102_1000, 0x1000),
            (0x0102_f000, 0x1000),
            (0x0103_c000, 0x1000),
            (0x0107_0000, 0x1000),
            (0x0107_3000, 0x1000),
        ] {
            e.map(base, size);
        }
        e.mem.set_f64(0x0102_f248, 57.295_776);
        e.mem.set_f64(0x0103_c348, 360.0);
        e.mem.set_f32(0x0107_3608, 180.0);
        e.mem.set_f64(0x0107_0c80, 0.15);
        e.mem.set_f64(0x0101_2070, 1.0);
        e.mem.set_f64(0x0101_1590, 2.0);
        e.mem.set_f64(0x0101_ff58, -std::f64::consts::PI);
        e.mem.set_f64(0x0101_ff48, 2.0 * std::f64::consts::PI);
        e.mem.set_f64(0x0101_ff40, std::f64::consts::PI);
        e.mem.set_f64(0x0107_34f8, 1.0);
        e.mem.set_f64(0x0102_17d8, 0.8);
        e.mem.set_f64(0x0101_e568, 255.0);
        e.mem.set_f32(0x011d_9774, 0.75);
        e.mem.set_f32(0x011c_d7d8, 40.0);
        e.mem.set_f32(0x011c_dcbc, 2.0);
        e.mem.set_i32(0x011d_12cc, 10);
        e.mem.set_i32(0x011d_0bd8, 5);
        e.mem.set_i32(0x011d_32d0, 6);
        e.mem.set_u8(0x011d_96c4, 1);
        let second_limit_setting = e.mem.alloc(4);
        e.mem.set_i32(second_limit_setting, 20);
        let player = e.mem.alloc(0x710);
        let position = e.mem.alloc(12);
        let compass = e.mem.alloc(4);
        let compass_target = e.mem.alloc(0x84);
        let landmarks = e.mem.alloc(8);
        let objectives = e.mem.alloc(8);
        let actor_ticks = e.mem.alloc(8);
        let hud = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_ea3c, player);
        e.mem.set_u32(0x011d_96c0, hud.addr());
        e.mem.set_i32(hud.addr() + 0x214, 100);
        let mut player_slots = [0_u32; 0xb0];
        player_slots[0x2bc / 4] = 0x0300_1010;
        player_slots[0x1f4 / 4] = 0x0300_1020;
        player_slots[0x22c / 4] = 0x0300_1030;
        e.put_vtable(0x0300_0000, &player_slots);
        e.mem.set_u32(player, 0x0300_0000);
        let mut compass_slots = [0_u32; 9];
        compass_slots[0x20 / 4] = 0x0300_2010;
        e.put_vtable(0x0300_2000, &compass_slots);
        e.mem.set_u32(compass, 0x0300_2000);
        e.register_double(0x0300_1010, |_, _| Ret {
            st0: 0.0,
            ..Ret::default()
        });
        e.register_double(0x0300_1020, move |_, _| Ret {
            eax: position,
            ..Ret::default()
        });
        e.register_double(0x0300_1030, |_, _| Ret::default());
        e.register_double(0x0300_2010, move |_, _| Ret {
            eax: compass_target,
            ..Ret::default()
        });
        e.register_double(0x00a0_0940, |_, words| Ret {
            eax: words[0],
            ..Ret::default()
        });
        e.register_double(0x008d_6f30, |_, _| Ret {
            eax: 0x4000,
            ..Ret::default()
        });
        e.register_double(0x0055_5ad0, |_, _| Ret {
            st0: 0.0,
            ..Ret::default()
        });
        e.register_double(0x0040_8840, |_, words| Ret {
            st0: f64::from(f32::from_bits(words[0]).abs()),
            ..Ret::default()
        });
        e.register_double(0x00ec_62c0, |_, _| Ret::default());
        e.register_double(0x004b_3ab0, |_, words| {
            assert_eq!(words.len(), 5);
            Ret {
                st0: 0.2,
                ..Ret::default()
            }
        });
        e.register_double(0x0094_8030, |_, _| Ret::default());
        e.register_double(0x0044_ddc0, |_, _| Ret::default());
        e.register_double(0x0040_3e20, |_, words| Ret {
            eax: words[0],
            ..Ret::default()
        });
        e.register_double(0x0055_9450, |_, _| Ret::default());
        e.register_double(0x0095_2ba0, |_, _| Ret {
            eax: 0x7777,
            ..Ret::default()
        });
        e.register_double(0x0072_6070, move |e, words| {
            if words[0] == objectives + 4 {
                e.mem.set_u32(hud.addr() + 0x20c, 0);
            }
            Ret::default()
        });
        e.register_double(0x0043_d4d0, move |_, words| Ret {
            eax: if words[0] == 0x011c_d7d8 {
                second_limit_setting
            } else {
                words[0]
            },
            ..Ret::default()
        });
        e.register_double(0x0059_5d10, |_, _| Ret::default());
        e.register_double(0x0042_5fd0, |_, _| Ret::default());
        e.register_double(0x005a_e380, |_, _| Ret::default());
        hud_main_menu_update_compass(
            &mut e,
            Ptr::new(compass),
            Ptr::new(landmarks),
            Ptr::new(objectives),
            Ptr::new(actor_ticks),
        );
        assert_eq!(e.mem.u32(0x011d_9e68), 7);
        assert_eq!(e.mem.u32(0x011d_9e64), 0x0107_3618);
        assert_eq!(e.mem.u32(0x011d_9e60), 0x0107_360c);
        assert_eq!(e.mem.u32(0x011d_9e5c), 0x0107_3500);
        assert!((e.mem.f32(compass_target + 0x7c) - 0.05).abs() < 0.0001);
        assert_eq!(e.mem.f32(compass_target + 0x80), 0.0);
        assert_eq!(e.mem.u32(hud.addr() + 0x20c), 0);
        assert_eq!(e.mem.u8(0x011d_96c4), 0);

        let actor = e.mem.alloc(0x200);
        let replacement_actor = e.mem.alloc(0x200);
        let actor_position = e.mem.alloc(12);
        e.mem.set_f32(actor_position, 3.0);
        e.mem.set_f32(actor_position + 4, 4.0);
        let record = e.mem.alloc(8);
        e.mem.set_u32(record, actor);
        e.mem.set_u8(record + 4, 1);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, record);
        let tile = e.mem.alloc(0x20);
        let tile_entry = e.mem.alloc(4);
        e.mem.set_u32(tile_entry, tile);
        let mut actor_slots = [0_u32; 0x80];
        actor_slots[0x1d0 / 4] = 0x0300_3010;
        actor_slots[0x1f4 / 4] = 0x0300_3020;
        e.put_vtable(0x0300_3000, &actor_slots);
        e.mem.set_u32(actor, 0x0300_3000);
        e.mem.set_u32(replacement_actor, 0x0300_3000);
        let mut actor_subobject_slots = [0_u32; 3];
        actor_subobject_slots[0x08 / 4] = 0x0300_3030;
        e.put_vtable(0x0300_3040, &actor_subobject_slots);
        e.mem.set_u32(actor + 0xa4, 0x0300_3040);
        e.register_double(0x0300_3010, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register_double(0x0300_3020, move |_, words| {
            assert_eq!(words[0], replacement_actor);
            Ret {
                eax: actor_position,
                ..Ret::default()
            }
        });
        e.register_double(0x0300_3030, |_, words| {
            assert!(words[1] == 0x31 || words[1] == 0x30);
            Ret::default()
        });
        e.register_double(0x0055_9450, move |_, words| Ret {
            eax: if words[0] == actor_ticks + 4 {
                0x1234
            } else {
                0
            },
            ..Ret::default()
        });
        e.register_double(0x0059_5d10, |_, _| Ret {
            eax: 0x5678,
            ..Ret::default()
        });
        e.register_double(0x005a_e380, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register_double(0x0082_56d0, |_, _| Ret::default());
        e.register_double(0x0068_15c0, move |_, _| Ret {
            eax: slot,
            ..Ret::default()
        });
        e.register_double(0x0045_0ff0, move |e, _| {
            e.mem.set_u32(record, replacement_actor);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.register_double(0x0043_9ef0, |_, words| Ret {
            eax: words[1],
            ..Ret::default()
        });
        e.register_double(0x004a_7290, |_, _| Ret {
            st0: 25.0,
            ..Ret::default()
        });
        e.register_double(0x004b_1550, |_, _| Ret {
            st0: 0.0,
            ..Ret::default()
        });
        e.register_double(0x0057_cbe0, move |e, words| {
            e.mem.set_u32(words[1], 0);
            Ret {
                eax: tile_entry,
                ..Ret::default()
            }
        });
        e.register_double(0x00a0_1350, |_, words| {
            assert_eq!(&words[1..], &[0xfcc, 0x0107_350c, 1]);
            Ret::default()
        });
        e.register_double(0x00a0_12d0, move |_, words| {
            assert_eq!(words[0], tile);
            Ret::default()
        });
        e.register_double(0x0070_0320, move |e, words| {
            assert_eq!(words[0], tile);
            if words[1] == 0xff4 {
                e.mem.set_u32(tile + 0x1c, words[2]);
            }
            Ret::default()
        });
        e.mem.set_i32(hud.addr() + 0x218, 180);
        e.call_log = Some(vec![]);
        hud_main_menu_update_compass(
            &mut e,
            Ptr::new(compass),
            Ptr::new(landmarks),
            Ptr::new(objectives),
            Ptr::new(actor_ticks),
        );
        assert_eq!(e.mem.u8(0x011d_96c4), 1);
        assert_eq!(e.mem.u32(tile + 0x1c), 2);
        let calls = e.call_log.as_ref().unwrap();
        assert!(calls.iter().any(|(addr, words)| {
            *addr == 0x00a0_12d0
                && words[0] == tile
                && words[1] == 0x0107_360c
                && (f32::from_bits(words[2]) - 2.2).abs() < 0.0001
        }));

        let first_marker = e.mem.alloc(4);
        let replacement_marker = e.mem.alloc(4);
        let replacement_position = e.mem.alloc(12);
        e.mem.set_u32(player + 0x6f4, first_marker);
        let mut marker_slots = [0_u32; 0x80];
        marker_slots[0x1f4 / 4] = 0x0300_4010;
        e.put_vtable(0x0300_4000, &marker_slots);
        e.mem.set_u32(replacement_marker, 0x0300_4000);
        e.register_double(0x0300_4010, move |_, words| {
            assert_eq!(words[0], replacement_marker);
            Ret {
                eax: replacement_position,
                ..Ret::default()
            }
        });
        e.register_double(0x0095_2ba0, |_, _| Ret::default());
        e.register_double(0x0072_6070, move |_, words| Ret {
            eax: if words[0] == objectives + 4 {
                0x3456
            } else {
                0
            },
            ..Ret::default()
        });
        e.register_double(0x0068_3520, move |e, words| {
            e.mem.set_u32(words[1], 0);
            Ret {
                eax: tile_entry,
                ..Ret::default()
            }
        });
        e.register_double(0x00a0_1350, move |e, words| {
            if words[2] == 0x0107_3534 {
                e.mem.set_u32(player + 0x6f4, replacement_marker);
            }
            Ret::default()
        });
        e.register_double(0x0055_9450, |_, _| Ret::default());
        e.register_double(0x0059_5d10, |_, _| Ret::default());
        e.mem.set_i32(hud.addr() + 0x218, 0);
        hud_main_menu_update_compass(
            &mut e,
            Ptr::new(compass),
            Ptr::new(landmarks),
            Ptr::new(objectives),
            Ptr::new(actor_ticks),
        );
        assert_eq!(e.mem.u32(player + 0x6f4), replacement_marker);

        let world_record = e.mem.alloc(8);
        let world_slot = e.mem.alloc(4);
        e.mem.set_u32(world_record, 0x6000);
        e.mem.set_u32(world_record + 4, first_marker);
        e.mem.set_u32(world_slot, world_record);
        e.mem.set_u32(player + 0x6f4, 0);
        e.register_double(0x0300_1030, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register_double(0x0094_8030, |_, _| Ret {
            eax: 0x9999,
            ..Ret::default()
        });
        e.register_double(0x0068_15c0, move |_, words| {
            assert_eq!(words[0], 0x9999);
            Ret {
                eax: world_slot,
                ..Ret::default()
            }
        });
        e.register_double(0x0072_6070, |_, _| Ret::default());
        e.register_double(0x0044_0da0, move |e, words| {
            assert_eq!(words[0], first_marker);
            e.mem.set_u32(world_record + 4, replacement_marker);
            Ret::default()
        });
        e.register_double(0x0043_8ef0, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register_double(0x0056_8cb0, |_, _| Ret {
            st0: 1.0,
            ..Ret::default()
        });
        hud_main_menu_update_compass(
            &mut e,
            Ptr::new(compass),
            Ptr::new(landmarks),
            Ptr::new(objectives),
            Ptr::new(actor_ticks),
        );
        assert_eq!(e.mem.u32(world_record + 4), replacement_marker);
    }

    /// `hud_main_menu_update_compass` with no landmarks, targets, objective
    /// tiles or VATS actors; every squared distance is 25. Returns the engine
    /// and the compass, landmark, objective and actor-tick pools.
    fn compass_engine() -> (Engine, [u32; 4]) {
        let mut e = Engine::new();
        for (base, size) in [
            (0x011d_0000, 0x6000),
            (0x011d_9000, 0x6000),
            (0x011c_d000, 0x1000),
            (0x0101_1000, 0x1000),
            (0x0101_e000, 0x1000),
            (0x0101_f000, 0x1000),
            (0x0102_1000, 0x1000),
            (0x0102_f000, 0x1000),
            (0x0103_c000, 0x1000),
            (0x0107_0000, 0x1000),
            (0x0107_3000, 0x1000),
        ] {
            e.map(base, size);
        }
        e.mem.set_f64(0x0102_f248, 57.295_776);
        e.mem.set_f64(0x0103_c348, 360.0);
        e.mem.set_f32(0x0107_3608, 180.0);
        e.mem.set_f64(0x0107_0c80, 0.15);
        e.mem.set_f64(0x0101_1590, 2.0);
        e.mem.set_f64(0x0101_ff58, -std::f64::consts::PI);
        e.mem.set_f64(0x0101_ff48, 2.0 * std::f64::consts::PI);
        e.mem.set_f64(0x0101_ff40, std::f64::consts::PI);
        e.mem.set_f64(0x0107_34f8, 1.0);
        e.mem.set_f64(0x0102_17d8, 0.8);
        e.mem.set_f64(0x0101_e568, 255.0);
        e.mem.set_f32(0x011d_9774, 0.75);
        e.mem.set_i32(0x011d_12cc, 10);
        e.mem.set_i32(0x011d_0bd8, 5);
        let player = e.mem.alloc(0x710);
        let position = e.mem.alloc(12);
        let compass = e.mem.alloc(4);
        let landmarks = e.mem.alloc(8);
        let objectives = e.mem.alloc(8);
        let actor_ticks = e.mem.alloc(8);
        let hud = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_ea3c, player);
        e.mem.set_u32(0x011d_96c0, hud.addr());
        e.mem.set_i32(hud.addr() + 0x214, 100);
        e.mem.set_i32(hud.addr() + 0x218, 180);
        let mut player_slots = [0_u32; 0x130];
        player_slots[0x2bc / 4] = 0x0300_1010;
        player_slots[0x1f4 / 4] = 0x0300_1020;
        player_slots[0x22c / 4] = 0x0300_1030;
        player_slots[0x488 / 4] = 0x0300_1040;
        e.put_vtable(0x0300_0000, &player_slots);
        e.mem.set_u32(player, 0x0300_0000);
        let mut compass_slots = [0_u32; 9];
        compass_slots[0x20 / 4] = 0x0300_2010;
        e.put_vtable(0x0300_2000, &compass_slots);
        e.mem.set_u32(compass, 0x0300_2000);
        e.register_double(0x0300_1010, |_, _| Ret::default());
        e.register_double(0x0300_1020, move |_, _| Ret {
            eax: position,
            ..Ret::default()
        });
        e.register_double(0x0300_1030, |_, _| Ret::default());
        e.register_double(0x0300_1040, |_, _| Ret::default());
        e.register_double(0x0300_2010, |_, _| Ret::default());
        e.register_double(0x00a0_0940, |_, words| Ret {
            eax: words[0],
            ..Ret::default()
        });
        e.register_double(0x008d_6f30, |_, _| Ret {
            eax: 0x4000,
            ..Ret::default()
        });
        e.register_double(0x0040_8840, |_, words| Ret {
            st0: f64::from(f32::from_bits(words[0]).abs()),
            ..Ret::default()
        });
        e.register_double(0x004b_3ab0, |_, words| {
            assert_eq!(words.len(), 5);
            Ret {
                st0: 0.2,
                ..Ret::default()
            }
        });
        e.register_double(0x0043_9ef0, |_, words| Ret {
            eax: words[1],
            ..Ret::default()
        });
        e.register_double(0x004a_7290, |_, _| Ret {
            st0: 25.0,
            ..Ret::default()
        });
        e.register_double(0x0040_3e20, |_, words| Ret {
            eax: words[0],
            ..Ret::default()
        });
        e.register_double(0x0043_d4d0, |_, words| Ret {
            eax: words[0],
            ..Ret::default()
        });
        for address in [
            0x0055_5ad0,
            0x00ec_62c0,
            0x0094_8030,
            0x0044_ddc0,
            0x0055_9450,
            0x0095_2ba0,
            0x0072_6070,
            0x004b_1550,
            0x0059_5d10,
            0x0042_5fd0,
            0x005a_e380,
        ] {
            e.register_double(address, |_, _| Ret::default());
        }
        (e, [compass, landmarks, objectives, actor_ticks])
    }

    fn run_compass(e: &mut Engine, pools: [u32; 4]) {
        hud_main_menu_update_compass(
            e,
            Ptr::new(pools[0]),
            Ptr::new(pools[1]),
            Ptr::new(pools[2]),
            Ptr::new(pools[3]),
        );
    }

    #[test]
    fn compass_landmark_keeps_location_byte_until_an_unchanged_travel_marker() {
        // State+5 ([EBP-0x8f]) is written only at 00779591 (1, changed) and 007795d6
        // (0, unchanged travel marker); an unchanged non-travel marker hands 00772aa0
        // the previous marker's byte.
        let (mut e, pools) = compass_engine();
        let marker_position = e.mem.alloc(12);
        let update = e.mem.alloc(0x324);
        let mut reference_slots = [0_u32; 0x7e];
        reference_slots[0x48 / 4] = 0x0300_3048;
        reference_slots[0x1f4 / 4] = 0x0300_31f4;
        e.put_vtable(0x0300_3000, &reference_slots);
        // Marker k: node (record, next, 011d9af0 seen at its turn), record (map data
        // 0x100 + k, reference).
        let mut nodes = Vec::new();
        let mut references = Vec::new();
        for index in 0..4_u32 {
            let reference = e.mem.alloc(8);
            e.mem.set_u32(reference, 0x0300_3000);
            let record = e.mem.alloc(8);
            e.mem.set_u32(record, 0x100 + index);
            e.mem.set_u32(record + 4, reference);
            let node = e.mem.alloc(12);
            e.mem.set_u32(node, record);
            nodes.push(node);
            references.push(reference);
        }
        for pair in nodes.windows(2) {
            e.mem.set_u32(pair[0] + 4, pair[1]);
        }
        let first = nodes[0];
        e.register_double(0x0094_8030, move |_, _| Ret {
            eax: first,
            ..Ret::default()
        });
        e.register_double(0x0068_15c0, |_, words| Ret {
            eax: words[0],
            ..Ret::default()
        });
        let objective_pool = pools[2] + 4;
        e.register_double(0x0072_6070, move |e, words| {
            if words[0] == objective_pool {
                return Ret::default();
            }
            // Keep 011d9af0 as the previous marker left it.
            let current = e.mem.u32(0x011d_9af0);
            e.mem.set_u32(words[0] + 8, current);
            Ret {
                eax: e.mem.u32(words[0] + 4),
                ..Ret::default()
            }
        });
        e.register_double(0x0044_0da0, |_, _| Ret::default());
        e.register_double(0x0300_31f4, move |_, _| Ret {
            eax: marker_position,
            ..Ret::default()
        });
        e.register_double(0x0300_3048, |_, _| Ret::default());
        // Only the third marker is a travel marker; only the first is in its radius.
        e.register_double(0x0043_8ef0, |_, words| Ret {
            eax: u32::from(words[0] == 0x102),
            ..Ret::default()
        });
        let inside = references[0];
        e.register_double(0x0056_8cb0, move |_, words| Ret {
            st0: if words[0] == inside { 10.0 } else { 1.0 },
            ..Ret::default()
        });
        e.register_double(0x0043_8ed0, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register_double(0x0043_8eb0, |_, _| Ret {
            eax: 0x7100,
            ..Ret::default()
        });
        e.register_double(0x0040_3df0, |_, _| Ret {
            eax: 0x7200,
            ..Ret::default()
        });
        e.register_double(0x0040_1000, move |_, _| Ret {
            eax: update,
            ..Ret::default()
        });
        for address in [
            0x0044_de80,
            0x0040_6d30,
            0x0076_bad0,
            0x005f_5950,
            0x004d_5c60,
        ] {
            e.register_double(address, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        run_compass(&mut e, pools);
        let calls = e.call_log.take().unwrap();
        // 00779509: 00438eb0 and 00403df0 take only `this`; the five pushed words
        // and `location` are 0076b960's.
        let location = calls
            .iter()
            .position(|(address, _)| *address == 0x0043_8eb0)
            .unwrap();
        assert_eq!(calls[location].1, args![0x100_u32].to_vec());
        assert_eq!(
            calls[location + 1],
            (0x0040_3df0, args![0x011d_2208_u32].to_vec())
        );
        assert_eq!(
            calls
                .iter()
                .filter(|(address, _)| *address == 0x0043_8eb0)
                .count(),
            1
        );
        // 011d9af0 after each marker.
        assert_eq!(e.mem.u32(nodes[1] + 8), references[0]);
        assert_eq!(e.mem.u32(nodes[2] + 8), references[1]);
        assert_eq!(e.mem.u32(nodes[3] + 8), references[1]);
        assert_eq!(e.mem.u32(0x011d_9af0), references[1]);
    }

    #[test]
    fn compass_actor_objective_marker_uses_max_alpha() {
        // 00779c4c: the actor branch sets 0xfa9 from [EBP-0x20] (max alpha); the
        // 004b3ab0 fade stored at [EBP-0xb4] is never read.
        let (mut e, pools) = compass_engine();
        let target = e.mem.alloc(8);
        let target_position = e.mem.alloc(12);
        let mut target_slots = [0_u32; 0x7e];
        target_slots[0x1f4 / 4] = 0x0300_41f4;
        e.put_vtable(0x0300_4000, &target_slots);
        e.mem.set_u32(target, 0x0300_4000);
        let target_record = e.mem.alloc(4);
        let target_node = e.mem.alloc(8);
        e.mem.set_u32(target_node, target_record);
        let enabled = e.mem.alloc(4);
        e.mem.set_u8(enabled, 1);
        let tile = e.mem.alloc(0x20);
        let tile_entry = e.mem.alloc(4);
        e.mem.set_u32(tile_entry, tile);
        e.register_double(0x0095_2ba0, move |_, _| Ret {
            eax: target_node,
            ..Ret::default()
        });
        let objective_pool = pools[2] + 4;
        e.register_double(0x0072_6070, move |_, words| Ret {
            eax: if words[0] == objective_pool {
                0x3456
            } else {
                0
            },
            ..Ret::default()
        });
        e.register_double(0x0068_15c0, |_, words| Ret {
            eax: words[0],
            ..Ret::default()
        });
        e.register_double(0x005c_bb70, move |_, words| {
            assert_eq!(words, &[target_record]);
            Ret {
                eax: target,
                ..Ret::default()
            }
        });
        e.register_double(0x0040_1170, |_, _| Ret {
            eax: 0x1c,
            ..Ret::default()
        });
        e.register_double(0x0040_8d60, move |_, _| Ret {
            eax: enabled,
            ..Ret::default()
        });
        e.register_double(0x0300_41f4, move |_, _| Ret {
            eax: target_position,
            ..Ret::default()
        });
        e.register_double(0x0040_ebd0, |_, _| Ret {
            st0: 5.0,
            ..Ret::default()
        });
        e.register_double(0x0068_3520, move |e, words| {
            e.mem.set_u32(words[1], 0);
            Ret {
                eax: tile_entry,
                ..Ret::default()
            }
        });
        e.register_double(0x00a0_3c90, |_, _| Ret {
            eax: 0x5000,
            ..Ret::default()
        });
        e.register_double(0x0059_bb30, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        for address in [0x00a0_1350, 0x00a0_12d0, 0x0070_0320] {
            e.register_double(address, |_, _| Ret::default());
        }
        e.call_log = Some(vec![]);
        run_compass(&mut e, pools);
        let calls = e.call_log.take().unwrap();
        // The fade calls still run (007799f3 / 00779a2e).
        assert!(calls.iter().any(|(address, _)| *address == 0x0040_ebd0));
        let alphas: Vec<_> = calls
            .iter()
            .filter(|(address, words)| *address == 0x00a0_12d0 && words[1] == 0xfa9)
            .map(|(_, words)| words.clone())
            .collect();
        // 0.75 opacity * 255.
        assert_eq!(alphas, [args![tile, 0xfa9_u32, 191.25_f32, 1_u32].to_vec()]);
    }

    #[test]
    fn square_helper_wraps_at_32_bits() {
        let mut e = Engine::new();
        assert_eq!(fn_0077f9c0(&mut e, -3), 9);
        assert_eq!(fn_0077f9c0(&mut e, 65_536), 0);
    }

    #[test]
    fn quest_text_waits_for_supported_hud_mode_and_debounce() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        let menu = e.new_object::<HUDMainMenu>();
        e.call_log = Some(vec![]);
        hud_main_menu_update_quest_text(&mut e, menu);
        assert!(e.call_log.as_ref().unwrap().is_empty());
        e.mem.set_u32(0x011d_96c0, menu.addr());
        e.mem.set_i32(menu.addr() + 0x1c4, 7);
        hud_main_menu_update_quest_text(&mut e, menu);
        assert!(e.call_log.as_ref().unwrap().is_empty());
        e.mem.set_i32(menu.addr() + 0x1c4, 0);
        e.register_double(0x0082_5c00, |_, words| {
            assert_eq!(words, &[0x011f_6394]);
            Ret {
                eax: 1999,
                ..Ret::default()
            }
        });
        hud_main_menu_update_quest_text(&mut e, menu);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn quest_popup_glyph_uses_uppercase_text_and_staggered_show_time() {
        let mut e = Engine::new();
        for (base, size) in [
            (0x011d_0000, 0x6000),
            (0x011d_9000, 0x6000),
            (0x0101_6000, 0x2000),
            (0x0102_0000, 0x1000),
        ] {
            e.map(base, size);
        }
        e.mem.set_f32(0x0101_6970, f32::MAX);
        e.mem.set_f64(0x0101_7b70, 1000.0);
        e.mem.set_f64(0x0102_0998, 5.0);
        e.mem.set_f32(0x011d_2688, 0.01);
        e.mem.set_f32(0x011d_4d14, 1.0);
        let menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, menu.addr());
        let pool = e.mem.alloc(8);
        e.mem.set_u32(menu.addr() + 0x60, pool);
        let head = e.mem.alloc(0x224);
        e.mem.set_u32(head + 4, 3);
        e.mem.set_i32(head + 0x21c, -1);
        e.mem.set_u8(head + 0x10c, b'a');
        e.mem.set_u8(head + 0x10d, 0);
        let head_slot = e.mem.alloc(4);
        e.mem.set_u32(head_slot, head);
        let glyph = e.mem.alloc(4);
        let glyph_slot = e.mem.alloc(4);
        e.mem.set_u32(glyph_slot, glyph);
        let glyph_text = e.mem.alloc(2);
        e.mem.set_u8(glyph_text, b'A');
        let taken = e.mem.alloc(4);
        e.register_double(0x0068_15c0, move |_, words| {
            assert_eq!(words, &[0x011d_970c]);
            Ret {
                eax: head_slot,
                ..Ret::default()
            }
        });
        e.register_double(0x0040_6d30, |e, words| {
            assert_eq!(words[1], 0x104);
            for index in 0..0x104_u32 {
                let value = e.mem.u8(words[2] + index);
                e.mem.set_u8(words[0] + index, value);
            }
            Ret::default()
        });
        e.register_double(0x00ec_a7f4, |_, words| Ret {
            eax: words[0] & !0x20,
            ..Ret::default()
        });
        e.register_double(0x0044_a670, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.register_double(0x005b_d5b0, |_, words| {
            assert!(words.is_empty());
            Ret {
                eax: 0x4400,
                ..Ret::default()
            }
        });
        e.register_double(0x00a1_b020, |e, words| {
            assert_eq!(words.len(), 6);
            assert_eq!(words[0], 0x4400);
            assert_eq!(words[5], 0);
            e.mem.set_f32(words[1], 10.0);
            e.mem.set_f32(words[1] + 4, 1.0);
            Ret::default()
        });
        e.register_double(0x00ec_62c0, |_, words| {
            let value = f64::from_bits(u64::from(words[0]) | (u64::from(words[1]) << 32));
            Ret {
                eax: value.trunc() as i32 as u32,
                ..Ret::default()
            }
        });
        e.register_double(0x0055_9450, move |_, words| {
            assert_eq!(words, &[pool + 4]);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.register_double(0x0057_cbe0, move |e, words| {
            let count = e.mem.u32(taken);
            e.mem.set_u32(taken, count + 1);
            e.mem.set_u32(words[1], if count == 0 { 2 } else { 0 });
            Ret {
                eax: glyph_slot,
                ..Ret::default()
            }
        });
        e.register_double(0x0070_0320, |_, _| Ret::default());
        e.register_double(0x00a0_1350, |e, words| {
            assert_eq!(e.mem.u8(words[2]), b'A');
            Ret::default()
        });
        e.register_double(0x00a0_11b0, |_, words| Ret {
            st0: match words[1] {
                0xfba => 1.0,
                0xfb9 => 8.0,
                0xfa1 => 0.0,
                _ => panic!("unexpected glyph trait"),
            },
            ..Ret::default()
        });
        e.register_double(0x0040_3e20, |_, words| Ret {
            eax: words[0],
            ..Ret::default()
        });
        e.register_double(0x0071_5c60, |_, _| Ret::default());
        e.register_double(0x00a0_12d0, |_, _| Ret::default());
        e.register_double(0x00a0_11f0, move |_, _| Ret {
            eax: glyph_text,
            ..Ret::default()
        });
        e.register_double(0x0040_fbe0, |_, _| Ret::default());
        e.register_double(0x0040_1030, |_, _| Ret::default());
        e.register_double(0x0063_f7b0, |_, _| Ret::default());
        e.register_double(0x0077_c170, |_, _| Ret::default());
        e.call_log = Some(vec![]);
        quest_popup_text(&mut e, menu, 100, 0.0, 0, 500);
        assert_eq!(e.mem.u32(taken), 2);
        let calls = e.call_log.as_ref().unwrap();
        assert!(calls.iter().any(|(address, words)| {
            *address == 0x0071_5c60 && words == &args![glyph, 0x1004_u32, 509_u32]
        }));
        assert!(calls.iter().any(|(address, words)| {
            *address == 0x00a0_12d0 && words[0] == glyph && words[1] == 0x1005
        }));
    }

    #[test]
    fn quest_popup_sound_getter_preserves_arguments_for_handle_lookup() {
        for (kind, name) in [(0_u32, 0x0106_f370), (3, 0), (2, 0x0107_3700)] {
            let mut e = Engine::new();
            e.map(0x011d_9000, 0x1000);
            let head = e.mem.alloc(0x224);
            e.mem.set_u32(head + 4, kind);
            let head_slot = e.mem.alloc(4);
            e.mem.set_u32(head_slot, head);
            let expected_name = if kind == 3 { head + 0x220 } else { name };
            e.register_double(0x0041_a250, |_, _| Ret::default());
            e.register_double(0x0068_15c0, move |_, _| Ret {
                eax: head_slot,
                ..Ret::default()
            });
            e.register_double(0x0045_3a70, |_, words| {
                assert!(words.is_empty());
                Ret {
                    eax: 0x4444,
                    ..Ret::default()
                }
            });
            e.register_double(0x00ad_7550, move |_, words| {
                assert_eq!(words.len(), 4);
                assert_eq!(words[0], 0x4444);
                assert_ne!(words[1], 0);
                assert_eq!(words[2], expected_name);
                assert_eq!(words[3], 0x121);
                Ret {
                    eax: 0x5555,
                    ..Ret::default()
                }
            });
            e.register_double(0x0041_8900, |_, words| {
                assert_eq!(words[1], 0x5555);
                std::panic::panic_any("stop after sound lookup")
            });
            let menu = e.new_object::<HUDMainMenu>();
            let stopped = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                quest_popup_start(&mut e, menu);
            }));
            assert_eq!(
                stopped.unwrap_err().downcast_ref::<&str>(),
                Some(&"stop after sound lookup")
            );
        }
    }

    #[test]
    fn quest_popup_show_time_uses_unsigned_trait_setter() {
        // 0077ac19: the popup's show time (trait 0x1004) goes through 00715c60 (FILD
        // qword of the zero-extended tick), not the signed 00700320.
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        e.map(0x0101_5000, 0x1000);
        let menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, menu.addr());
        let tile_list = e.mem.alloc(8);
        e.mem.set_u32(menu.addr() + 0x60, tile_list);
        let tile_slot = e.mem.alloc(4);
        e.mem.set_u32(tile_slot, 0x4000);
        let head = e.mem.alloc(0x224);
        let head_slot = e.mem.alloc(4);
        e.mem.set_u32(head_slot, head);
        for address in [
            0x0041_a250,
            0x0045_3a70,
            0x00ad_7550,
            0x0041_8900,
            0x0048_3710,
            0x00ad_8830,
            0x0040_3df0,
            0x00a0_1350,
            0x0070_0320,
            0x00a0_11b0,
            0x00ec_62c0,
            0x00a0_11f0,
            0x005b_d5b0,
            0x00a1_b020,
            0x00a0_12d0,
            0x0071_5d40,
            0x0071_77c0,
        ] {
            e.register_double(address, |_, _| Ret::default());
        }
        e.register_double(0x0068_15c0, move |_, _| Ret {
            eax: head_slot,
            ..Ret::default()
        });
        e.register_double(0x0082_5c00, |_, _| Ret {
            eax: 0x8000_0010,
            ..Ret::default()
        });
        e.register_double(0x0073_ac90, move |_, words| {
            assert_eq!(words, &[tile_list + 4]);
            Ret {
                eax: tile_slot,
                ..Ret::default()
            }
        });
        e.register_double(0x0071_5c60, |_, _| {
            std::panic::panic_any("stop at show time")
        });
        e.call_log = Some(vec![]);
        let stopped = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            quest_popup_start(&mut e, menu);
        }));
        assert_eq!(
            stopped.unwrap_err().downcast_ref::<&str>(),
            Some(&"stop at show time")
        );
        let calls = e.call_log.take().unwrap();
        assert_eq!(
            calls.last().unwrap(),
            &(
                0x0071_5c60,
                args![0x4000_u32, 0x1004_u32, 0x8000_0010_u32].to_vec()
            )
        );
        assert!(calls
            .iter()
            .all(|(address, words)| *address != 0x0070_0320 || words[1] != 0x1004));
    }

    #[test]
    fn quest_queue_head_reads_fixed_queue() {
        let mut e = Engine::new();
        let slot = e.mem.alloc(4);
        e.mem.set_u32(slot, 0x4567);
        e.register_double(0x0068_15c0, move |_, words| {
            assert_eq!(words, &[0x011d_970c]);
            Ret {
                eax: slot,
                ..Ret::default()
            }
        });
        assert_eq!(fn_0077b2a0(&mut e, Ptr::new(0x1234)).addr(), 0x4567);
    }

    #[test]
    fn quest_queue_empty_ignores_received_this() {
        let mut e = Engine::new();
        e.register_double(0x0082_56d0, |_, words| {
            assert_eq!(words, &[0x011d_970c]);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        assert!(fn_0077b2c0(&mut e, Ptr::new(0x1234)));
    }

    #[test]
    fn quest_queue_pop_uses_fixed_queue() {
        let mut e = Engine::new();
        e.register_double(0x0063_f7b0, |_, words| {
            assert_eq!(words, &[0x011d_970c]);
            Ret::default()
        });
        fn_0077b2e0(&mut e, Ptr::new(0x1234));
    }

    #[test]
    fn vats_distance_comparator_orders_and_handles_ties() {
        let mut e = Engine::new();
        e.map(0x011d_e000, 0x1000);
        let player = e.mem.alloc(4);
        let first_actor = e.mem.alloc(4);
        let second_actor = e.mem.alloc(4);
        let player_position = e.mem.alloc(12);
        let first_position = e.mem.alloc(12);
        let second_position = e.mem.alloc(12);
        let first_slot = e.mem.alloc(4);
        let second_slot = e.mem.alloc(4);
        e.mem.set_u32(0x011d_ea3c, player);
        e.mem.set_u32(first_slot, first_actor);
        e.mem.set_u32(second_slot, second_actor);
        let mut slots = [0_u32; 0x80];
        slots[0x1f4 / 4] = 0x0300_5010;
        e.put_vtable(0x0300_5000, &slots);
        for actor in [player, first_actor, second_actor] {
            e.mem.set_u32(actor, 0x0300_5000);
        }
        e.mem.set_f32(first_position, 3.0);
        e.mem.set_f32(second_position, 2.0);
        e.register_double(0x0300_5010, move |_, words| Ret {
            eax: match words[0] {
                value if value == player => player_position,
                value if value == first_actor => first_position,
                value if value == second_actor => second_position,
                _ => panic!("unknown actor"),
            },
            ..Ret::default()
        });
        e.register_double(0x0043_9ef0, |e, words| {
            e.mem.set_f32(words[1], e.mem.f32(words[0]));
            Ret {
                eax: words[1],
                ..Ret::default()
            }
        });
        e.register_double(0x004a_7290, |e, words| {
            let x = e.mem.f32(words[0]);
            Ret {
                st0: f64::from(x * x),
                ..Ret::default()
            }
        });
        e.register_double(0x0040_8840, |_, words| Ret {
            st0: f64::from(f32::from_bits(words[0]).abs()),
            ..Ret::default()
        });
        assert_eq!(
            fn_0077b300(&mut e, Ptr::new(first_slot), Ptr::new(second_slot)),
            1
        );
        e.mem.set_f32(first_position, 1.0);
        assert_eq!(
            fn_0077b300(&mut e, Ptr::new(first_slot), Ptr::new(second_slot)),
            -1
        );
        e.mem.set_f32(first_position, 2.0);
        assert_eq!(
            fn_0077b300(&mut e, Ptr::new(first_slot), Ptr::new(second_slot)),
            0
        );
        e.mem.set_f32(first_position, f32::NAN);
        assert_eq!(
            fn_0077b300(&mut e, Ptr::new(first_slot), Ptr::new(second_slot)),
            0
        );
        assert_eq!(fn_0077b300(&mut e, Ptr::new(0), Ptr::new(second_slot)), 0);
    }

    #[test]
    fn reminder_fade_skips_first_31_tiles_before_empty_queue() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        let menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, menu.addr());
        let pool = e.mem.alloc(8);
        e.mem.set_u32(menu.addr() + 0x5c, pool);
        e.register_double(0x0071_5d40, |_, _| Ret {
            st0: 1920.0,
            ..Ret::default()
        });
        e.register_double(0x00ec_62c0, |_, _| Ret {
            eax: 1920,
            ..Ret::default()
        });
        e.register_double(0x0055_9450, move |_, words| {
            assert_eq!(words, &[pool + 4]);
            Ret::default()
        });
        e.register_double(0x0057_cbe0, move |_, words| {
            assert_eq!(words[0], pool + 4);
            Ret::default()
        });
        e.register_double(0x0068_3520, move |_, words| {
            assert_eq!(words[0], pool + 4);
            Ret::default()
        });
        e.register_double(0x0072_6070, move |_, words| {
            assert_eq!(words, &[pool + 4]);
            Ret::default()
        });
        e.register_double(0x00a0_0940, |_, words| {
            assert_eq!(words, &[0x0107_3724, u32::MAX]);
            Ret {
                eax: 0x1234,
                ..Ret::default()
            }
        });
        e.register_double(0x0082_56d0, |_, _| Ret {
            eax: 1,
            ..Ret::default()
        });
        e.call_log = Some(vec![]);
        hud_main_menu_handle_reminder_fade(&mut e, menu, Ptr::new(pool));
        assert_eq!(
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|(address, _)| *address == 0x0057_cbe0)
                .count(),
            31
        );
        assert_eq!(e.mem.u32(0x011d_9e70), 0x1234);
    }

    #[test]
    fn reminder_fade_places_queued_tile_with_sound_and_reflows_width() {
        use std::cell::Cell;
        use std::rc::Rc;

        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        e.map(0x0101_0000, 0x20000);
        e.mem.set_f64(0x0102_fc70, 20.0);
        let menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, menu.addr());
        let pool = e.mem.alloc(8);
        e.mem.set_u32(menu.addr() + 0x5c, pool);
        let marker_tile = e.mem.alloc(4);
        let icon_tile = e.mem.alloc(4);
        let marker_slot = e.mem.alloc(4);
        let icon_slot = e.mem.alloc(4);
        e.mem.set_u32(marker_slot, marker_tile);
        e.mem.set_u32(icon_slot, icon_tile);
        let text = e.mem.alloc(8);
        let record = e.mem.alloc(0x110);
        let record_slot = e.mem.alloc(4);
        e.mem.set_u32(record_slot, record);
        let removed = Rc::new(Cell::new(false));
        let removed_for_empty = removed.clone();
        e.register_double(0x0082_56d0, move |_, words| {
            assert_eq!(words, &[menu.addr() + 0x258]);
            Ret {
                eax: u32::from(removed_for_empty.get()),
                ..Ret::default()
            }
        });
        e.register_double(0x0068_15c0, move |_, words| {
            assert_eq!(words, &[menu.addr() + 0x258]);
            Ret {
                eax: record_slot,
                ..Ret::default()
            }
        });
        let removed_for_pop = removed.clone();
        e.register_double(0x0090_5330, move |_, words| {
            assert_eq!(words, &[menu.addr() + 0x258, record_slot]);
            removed_for_pop.set(true);
            Ret::default()
        });
        e.register_double(0x0071_5d40, |_, words| {
            assert!(words.is_empty());
            Ret {
                st0: 1920.0,
                ..Ret::default()
            }
        });
        e.register_double(0x00ec_62c0, |_, words| {
            let value = f64::from_bits(u64::from(words[0]) | (u64::from(words[1]) << 32));
            Ret {
                eax: value as i32 as u32,
                ..Ret::default()
            }
        });
        let mut first_calls = 0;
        e.register_double(0x0055_9450, move |_, words| {
            assert_eq!(words, &[pool + 4]);
            first_calls += 1;
            Ret {
                eax: if first_calls < 3 { 1 } else { 0 },
                ..Ret::default()
            }
        });
        let mut icon_first_calls = 0;
        e.register_double(0x0072_6070, move |_, words| {
            assert_eq!(words, &[pool + 4]);
            icon_first_calls += 1;
            Ret {
                eax: if icon_first_calls < 3 { 2 } else { 0 },
                ..Ret::default()
            }
        });
        let mut advances = 0;
        e.register_double(0x0057_cbe0, move |e, words| {
            assert_eq!(words[0], pool + 4);
            advances += 1;
            if advances > 31 {
                e.mem.set_u32(words[1], 0);
                Ret {
                    eax: if advances == 32 {
                        marker_slot
                    } else {
                        icon_slot
                    },
                    ..Ret::default()
                }
            } else {
                Ret::default()
            }
        });
        let mut takes = 0;
        e.register_double(0x0068_3520, move |_, words| {
            assert_eq!(words[0], pool + 4);
            takes += 1;
            Ret {
                eax: match takes {
                    1 => marker_slot,
                    2 => icon_slot,
                    _ => 0,
                },
                ..Ret::default()
            }
        });
        e.register_double(0x00a0_1860, |_, words| {
            assert_eq!(words, &[0x0107_37b4]);
            Ret {
                eax: 0x1234,
                ..Ret::default()
            }
        });
        e.register_double(0x00a0_1230, move |_, words| {
            assert_eq!(words, &[marker_tile, 0xfa3]);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.register_double(0x00a0_1350, move |_, words| {
            assert_eq!(words[3], 1);
            assert!(
                words == [marker_tile, 0xfc4, record, 1]
                    || words == [icon_tile, 0xfcc, 0x0107_374c, 1]
            );
            Ret::default()
        });
        e.register_double(0x0070_0320, |_, _| Ret::default());
        e.register_double(0x00a0_11b0, move |_, words| Ret {
            st0: match words {
                [tile, 0xfa1] if *tile == marker_tile => 30.0,
                [tile, 0xfba] if *tile == marker_tile => 1.0,
                [tile, 0xfb9] if *tile == marker_tile => 7.0,
                _ => panic!("unexpected float trait"),
            },
            ..Ret::default()
        });
        e.register_double(0x00a0_12d0, move |_, words| {
            assert_eq!(words[0..2], [icon_tile, 0xfa1]);
            assert_eq!(f32::from_bits(words[2]), 10.0);
            assert_eq!(words[3], 1);
            Ret::default()
        });
        e.register_double(0x0040_6d00, |_, words| {
            assert_eq!(words[1..], [0x104, 0x0107_3744, 0]);
            Ret::default()
        });
        e.register_double(0x0070_7e10, move |_, words| {
            assert_eq!(words[0], marker_tile);
            Ret::default()
        });
        e.register_double(0x0045_3a70, |_, words| {
            assert!(words.is_empty());
            Ret {
                eax: 0x5555,
                ..Ret::default()
            }
        });
        e.register_double(0x00ad_7550, |_, words| {
            assert_eq!(words[0], 0x5555);
            assert_ne!(words[1], 0);
            assert_eq!(words[2..], [0x0107_3734, 0x121]);
            Ret {
                eax: 0x6666,
                ..Ret::default()
            }
        });
        e.register_double(0x00ad_8830, |_, words| {
            assert_eq!(words, &[0x6666, 0]);
            Ret::default()
        });
        e.register_double(0x0048_3710, |_, _| Ret::default());
        e.register_double(0x00a0_11f0, move |_, words| {
            assert_eq!(words, &[marker_tile, 0xfc4]);
            Ret {
                eax: text,
                ..Ret::default()
            }
        });
        e.register_double(0x005b_d5b0, |_, words| {
            assert!(words.is_empty());
            Ret {
                eax: 0x7777,
                ..Ret::default()
            }
        });
        e.register_double(0x00a1_b020, move |e, words| {
            assert_eq!(words[0], 0x7777);
            assert_eq!(words[2..], [text, 7, 1.0_f32.to_bits(), 0]);
            e.mem.set_f32(words[1] + 4, 30.0);
            Ret::default()
        });
        e.register_double(0x00a0_0940, |_, words| {
            assert_eq!(words, &[0x0107_3724, u32::MAX]);
            Ret {
                eax: 0x8888,
                ..Ret::default()
            }
        });
        hud_main_menu_handle_reminder_fade(&mut e, menu, Ptr::new(pool));
        assert!(removed.get());
        assert_eq!(e.mem.u8(menu.addr() + 0x261), 1);
        assert_eq!(e.mem.u32(0x011d_9e70), 0x8888);
    }

    #[test]
    fn reminder_fade_reloads_alpha_and_wait_trait_after_callback() {
        use std::cell::RefCell;
        use std::rc::Rc;

        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        e.map(0x0101_0000, 0x20000);
        e.mem.set_f64(0x0101_2060, 0.0);
        e.mem.set_f32(0x0101_6248, 0.5);
        e.mem.set_f64(0x0102_1928, 3.0);
        e.mem.set_f32(0x011d_979c, 0.75);
        let menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, menu.addr());
        let pool = e.mem.alloc(8);
        e.mem.set_u32(menu.addr() + 0x5c, pool);
        let marker = e.mem.alloc(4);
        let icon = e.mem.alloc(4);
        let marker_slot = e.mem.alloc(4);
        let icon_slot = e.mem.alloc(4);
        e.mem.set_u32(marker_slot, marker);
        e.mem.set_u32(icon_slot, icon);
        let seconds = e.mem.alloc(4);
        e.mem.set_f32(seconds, 1.0);
        e.register_double(0x0071_5d40, |_, _| Ret {
            st0: 1920.0,
            ..Ret::default()
        });
        e.register_double(0x00ec_62c0, |_, _| Ret {
            eax: 1920,
            ..Ret::default()
        });
        let mut first_calls = 0;
        e.register_double(0x0055_9450, move |_, words| {
            assert_eq!(words, &[pool + 4]);
            first_calls += 1;
            Ret {
                eax: if first_calls == 1 { 0 } else { 1 },
                ..Ret::default()
            }
        });
        let mut icon_first_calls = 0;
        e.register_double(0x0072_6070, move |_, words| {
            assert_eq!(words, &[pool + 4]);
            icon_first_calls += 1;
            Ret {
                eax: if icon_first_calls == 1 { 0 } else { 1 },
                ..Ret::default()
            }
        });
        let mut advances = 0;
        e.register_double(0x0057_cbe0, move |e, words| {
            assert_eq!(words[0], pool + 4);
            advances += 1;
            if advances <= 31 {
                Ret::default()
            } else {
                e.mem.set_u32(words[1], 0);
                Ret {
                    eax: if advances == 32 {
                        marker_slot
                    } else {
                        icon_slot
                    },
                    ..Ret::default()
                }
            }
        });
        e.register_double(0x0068_3520, move |_, words| {
            assert_eq!(words[0], pool + 4);
            Ret::default()
        });
        e.register_double(0x0082_56d0, move |_, words| {
            assert_eq!(words, &[menu.addr() + 0x258]);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.register_double(0x00a0_0940, |_, words| {
            assert_eq!(words, &[0x0107_3724, u32::MAX]);
            Ret {
                eax: 0x1111,
                ..Ret::default()
            }
        });
        e.register_double(0x00a0_1230, move |_, words| {
            assert_eq!(words, &[marker, 0xfa3]);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.register_double(0x00a0_11b0, move |_, words| {
            assert_eq!(words, &[marker, 0x1111]);
            Ret::default()
        });
        e.register_double(0x0070_0320, move |_, words| {
            assert_eq!(words, &[marker, 0x1111, 0]);
            Ret::default()
        });
        e.register_double(0x0040_3e20, move |_, words| {
            assert_eq!(words, &[0x011d_4cc0]);
            Ret {
                eax: seconds,
                ..Ret::default()
            }
        });
        let calls = Rc::new(RefCell::new(Vec::<Vec<u32>>::new()));
        let calls_for_double = calls.clone();
        e.register_double(0x00a0_7c60, move |e, words| {
            calls_for_double.borrow_mut().push(words.to_vec());
            if calls_for_double.borrow().len() == 1 {
                e.mem.set_f32(0x011d_979c, 0.25);
                e.mem.set_u32(0x011d_9e70, 0x2222);
            }
            Ret::default()
        });
        hud_main_menu_handle_reminder_fade(&mut e, menu, Ptr::new(pool));
        let calls = calls.borrow();
        assert_eq!(calls.len(), 3);
        assert_eq!(
            calls[0],
            vec![marker, 0xfa9, 0, 0.75_f32.to_bits(), 0.5_f32.to_bits(), 0]
        );
        assert_eq!(calls[1][0..2], [marker, 0x2222]);
        assert_eq!(
            calls[2],
            vec![icon, 0xfa9, 0, 0.25_f32.to_bits(), 0.5_f32.to_bits(), 0]
        );
    }

    #[test]
    fn cinematic_fade_starts_and_ends_text_after_clock_deadlines() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        e.map(0x0101_0000, 0x70000);
        e.mem.set_f64(0x0101_2060, 0.0);
        e.mem.set_f64(0x0101_e568, 255.0);
        e.mem.set_f64(0x0107_37c0, 175.0);
        let menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, menu.addr());
        let pool = e.mem.alloc(8);
        let cinematic_list = e.mem.alloc(8);
        e.mem.set_u32(menu.addr() + 0x5c, pool);
        let marker = e.mem.alloc(4);
        let start_tile = e.mem.alloc(4);
        let end_tile = e.mem.alloc(4);
        let marker_slot = e.mem.alloc(4);
        let start_slot = e.mem.alloc(4);
        let end_slot = e.mem.alloc(4);
        e.mem.set_u32(marker_slot, marker);
        e.mem.set_u32(start_slot, start_tile);
        e.mem.set_u32(end_slot, end_tile);
        let text = e.mem.alloc(8);
        let start_duration = e.mem.alloc(4);
        let end_duration = e.mem.alloc(4);
        let alpha_setting = e.mem.alloc(4);
        e.mem.set_f32(start_duration, 0.5);
        e.mem.set_f32(end_duration, 1.5);
        e.mem.set_f32(alpha_setting, 1.0);
        e.register_double(0x00a0_11b0, move |_, words| Ret {
            st0: match words {
                [tile, 0xfb6] if *tile == cinematic_list => 1.0,
                [tile, 0x1004] if *tile == start_tile => 5.0,
                [tile, 0x1004] if *tile == end_tile => 0.0,
                [tile, 0x1005] if *tile == end_tile => 10.0,
                [tile, 0xfa9] if *tile == end_tile => 200.0,
                _ => panic!("unexpected cinematic float trait"),
            },
            ..Ret::default()
        });
        e.register_double(0x0055_9450, move |_, words| Ret {
            eax: if words == [pool + 4] {
                1
            } else if words == [cinematic_list + 4] {
                2
            } else {
                panic!("unexpected cinematic list")
            },
            ..Ret::default()
        });
        e.register_double(0x0057_cbe0, move |e, words| {
            if words[0] == pool + 4 {
                e.mem.set_u32(words[1], 0);
                Ret {
                    eax: marker_slot,
                    ..Ret::default()
                }
            } else {
                assert_eq!(words[0], cinematic_list + 4);
                let cursor = e.mem.u32(words[1]);
                e.mem.set_u32(words[1], if cursor == 2 { 3 } else { 0 });
                Ret {
                    eax: if cursor == 2 { start_slot } else { end_slot },
                    ..Ret::default()
                }
            }
        });
        e.register_double(0x00a0_12d0, move |_, words| {
            assert_eq!(words, &[marker, 0xfa9, 0.0_f32.to_bits(), 1]);
            Ret::default()
        });
        e.register_double(0x00a0_1860, |_, words| {
            assert_eq!(words, &[0x0107_3724]);
            Ret {
                eax: 0x5555,
                ..Ret::default()
            }
        });
        e.register_double(0x0070_0320, move |_, words| {
            assert!(
                words == [marker, 0x5555, 0]
                    || words == [start_tile, 0x1004, 0]
                    || words == [end_tile, 0x1005, 0]
            );
            Ret::default()
        });
        e.register_double(0x0082_5c00, |_, words| {
            assert_eq!(words, &[0x011f_6394]);
            Ret {
                eax: 100,
                ..Ret::default()
            }
        });
        e.register_double(0x00a0_11f0, move |_, words| {
            assert_eq!(words, &[start_tile, 0xfc4]);
            Ret {
                eax: text,
                ..Ret::default()
            }
        });
        e.register_double(0x0070_7e10, move |_, words| {
            assert_eq!(words, &[start_tile, text]);
            Ret::default()
        });
        e.register_double(0x0040_3e20, move |_, words| Ret {
            eax: match words {
                [0x011d_2238] => start_duration,
                [0x011d_9774] => alpha_setting,
                [0x011d_4f30] => end_duration,
                _ => panic!("unexpected cinematic setting getter"),
            },
            ..Ret::default()
        });
        e.register_double(0x00a0_7c60, move |_, words| {
            assert_eq!(words[1], 0xfa9);
            if words[0] == start_tile {
                assert_eq!(
                    words[2..],
                    [0.0_f32.to_bits(), 255.0_f32.to_bits(), 0.5_f32.to_bits(), 0]
                );
            } else {
                assert_eq!(words[0], end_tile);
                assert_eq!(
                    words[2..],
                    [200.0_f32.to_bits(), 0.0_f32.to_bits(), 1.5_f32.to_bits(), 0]
                );
            }
            Ret::default()
        });
        hud_main_menu_handle_cinematic_fade(&mut e, menu, Ptr::new(cinematic_list));
    }

    #[test]
    fn timed_hud_value_uses_low_16_bits_and_reloadable_list_owner() {
        use std::cell::Cell;
        use std::rc::Rc;

        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        e.map(0x011d_e000, 0x2000);
        let menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, menu.addr());
        let player = e.mem.alloc(0xa8);
        e.mem.set_u32(0x011d_ea3c, player);
        let mut vtable = [0_u32; 4];
        vtable[3] = 0x0300_6600;
        e.put_vtable(0x0300_6500, &vtable);
        e.mem.set_u32(player + 0xa4, 0x0300_6500);
        let setting = e.mem.alloc(4);
        e.mem.set_i32(setting, 100);
        e.register_double(0x0087_f9f0, move |_, words| {
            assert_eq!(words, &[player]);
            Ret {
                eax: 0xabcd_0002,
                ..Ret::default()
            }
        });
        e.register_double(0x0043_d4d0, move |_, words| {
            assert_eq!(words, &[0x011d_0c60]);
            Ret {
                eax: setting,
                ..Ret::default()
            }
        });
        let allocations = Rc::new(Cell::new(0));
        let allocations_for_double = allocations.clone();
        e.register_double(0x0040_1000, move |e, words| {
            assert_eq!(words, &[12]);
            allocations_for_double.set(allocations_for_double.get() + 1);
            Ret {
                eax: e.mem.alloc(12),
                ..Ret::default()
            }
        });
        e.register_double(0x0300_6600, move |_, words| {
            assert_eq!(words, &[player + 0xa4, 0x18]);
            Ret {
                st0: 3.6,
                ..Ret::default()
            }
        });
        e.register_double(0x00ec_62c0, |_, _| Ret {
            eax: 4,
            ..Ret::default()
        });
        e.register_double(0x005a_e3d0, move |e, words| {
            assert_eq!(words[0], menu.addr() + 0x264);
            let record = e.mem.u32(words[1]);
            assert_eq!(e.mem.u32(record), 2);
            assert_eq!(e.mem.u32(record + 4), 4);
            assert_eq!(e.mem.u32(record + 8), 0x1234);
            Ret::default()
        });
        fn_0077c440(&mut e, 0x1234);
        e.mem.set_u8(menu.addr() + 0x204, 1);
        fn_0077c440(&mut e, 0x5678);
        assert_eq!(allocations.get(), 1);
    }

    #[test]
    fn xp_meter_inactive_mode_hides_six_tiles_then_zero_alpha_blocks_display() {
        use std::cell::Cell;
        use std::rc::Rc;

        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        e.map(0x011d_e000, 0x2000);
        e.map(0x011a_0000, 0x2000);
        let menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, menu.addr());
        let player = e.mem.alloc(4);
        e.mem.set_u32(0x011d_ea3c, player);
        let mut tiles = [0_u32; 6];
        for (index, offset) in [0xd8, 0xdc, 0xe0, 0xe4, 0xe8, 0xec].into_iter().enumerate() {
            tiles[index] = e.mem.alloc(4);
            e.mem.set_u32(menu.addr() + offset, tiles[index]);
        }
        e.register_double(0x00a0_0940, |_, words| Ret {
            eax: match words {
                [0x0107_37fc, value] if *value == u32::MAX => 0x1111,
                [0x0107_37f4, value] if *value == u32::MAX => 0x2222,
                _ => panic!("unexpected XP trait name"),
            },
            ..Ret::default()
        });
        e.register_double(0x0095_3c20, move |_, words| {
            assert_eq!(words, &[player]);
            Ret::default()
        });
        e.register_double(0x0046_4e10, move |_, words| {
            assert_eq!(words, &[player]);
            Ret {
                eax: 0x3333,
                ..Ret::default()
            }
        });
        e.register_double(0x008d_51f0, |_, words| {
            assert_eq!(words, &[0x3333]);
            Ret::default()
        });
        let menu_mode = Rc::new(Cell::new(true));
        let menu_mode_for_double = menu_mode.clone();
        e.register_double(0x0070_2360, move |_, words| {
            assert!(words.is_empty());
            Ret {
                eax: u32::from(menu_mode_for_double.get()),
                ..Ret::default()
            }
        });
        e.register_double(0x0070_23c0, |_, words| {
            assert!(words.is_empty());
            Ret {
                eax: 0x400,
                ..Ret::default()
            }
        });
        e.register_double(0x0044_ddc0, |_, words| {
            assert_eq!(words, &[0x011f_2250]);
            Ret::default()
        });
        e.register_double(0x0070_1740, move |_, words| {
            assert_eq!(words, &[menu.addr() + 0x1c8]);
            Ret::default()
        });
        e.register_double(0x0070_0320, move |_, words| {
            assert!(tiles.contains(&words[0]));
            assert_eq!(words[1..], [0xfa3, 0]);
            Ret::default()
        });
        e.register_double(0x00a0_12d0, move |_, words| {
            assert_eq!(words, &[tiles[0], 0xfa9, 0.0_f32.to_bits(), 1]);
            Ret::default()
        });
        e.register_double(0x00a0_7dc0, move |_, words| {
            assert_eq!(words, &[tiles[0], 0xfa9]);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        hud_main_menu_update_xp_meter(&mut e, menu);
        assert_eq!(e.mem.u32(0x011d_9e84), 0x1111);
        assert_eq!(e.mem.u32(0x011d_9e80), 0x2222);
        assert_eq!(e.mem.u32(0x011a_010c) & 1, 1);
        assert_eq!(e.mem.u8(menu.addr() + 0x204), 0);
        assert_eq!(
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|(address, _)| *address == 0x0070_0320)
                .count(),
            6
        );
        menu_mode.set(false);
        hud_main_menu_update_xp_meter(&mut e, menu);
        assert_eq!(e.mem.u8(menu.addr() + 0x204), 1);
    }

    #[test]
    fn xp_meter_level_up_popup_preserves_sound_and_tile_sequence() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        e.map(0x011d_e000, 0x2000);
        e.map(0x011a_0000, 0x2000);
        e.map(0x0101_0000, 0x10000);
        e.mem.set_f64(0x0101_2060, 0.0);
        e.mem.set_f32(0x0101_6248, 0.5);
        e.mem.set_f32(0x011d_979c, 255.0);
        e.mem.set_u32(0x011a_010c, 2);
        let menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, menu.addr());
        let meter = e.mem.alloc(4);
        let popup = e.mem.alloc(4);
        e.mem.set_u32(menu.addr() + 0xd8, meter);
        e.mem.set_u32(menu.addr() + 0xf0, popup);
        let player = e.mem.alloc(4);
        e.mem.set_u32(0x011d_ea3c, player);
        e.register_double(0x00a0_0940, |_, words| Ret {
            eax: match words[0] {
                0x0107_37fc => 0x1111,
                0x0107_37f4 => 0x2222,
                _ => panic!("unexpected XP trait"),
            },
            ..Ret::default()
        });
        e.register_double(0x0095_3c20, move |_, words| {
            assert_eq!(words, &[player]);
            Ret::default()
        });
        e.register_double(0x0046_4e10, move |_, words| {
            assert_eq!(words, &[player]);
            Ret {
                eax: 0x3333,
                ..Ret::default()
            }
        });
        e.register_double(0x008d_51f0, |_, words| {
            assert_eq!(words, &[0x3333]);
            Ret::default()
        });
        e.register_double(0x0070_2360, |_, words| {
            assert!(words.is_empty());
            Ret::default()
        });
        e.register_double(0x0044_ddc0, |_, words| {
            assert_eq!(words, &[0x011f_2250]);
            Ret::default()
        });
        e.register_double(0x0070_1740, move |_, words| {
            assert_eq!(words, &[menu.addr() + 0x1c8]);
            Ret::default()
        });
        e.register_double(0x00a0_11b0, move |_, words| {
            assert!(words == [meter, 0xfa9] || words == [popup, 0xfa9]);
            Ret::default()
        });
        e.register_double(0x00a0_7fc0, move |_, words| {
            assert!(words == [meter, 0xfa9] || words == [popup, 0xfa9]);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.register_double(0x005c_7870, move |_, words| {
            assert_eq!(words, &[player]);
            Ret::default()
        });
        e.register_double(0x00a0_1230, move |_, words| {
            assert_eq!(words, &[popup, 0x100e]);
            Ret::default()
        });
        e.register_double(0x0045_3a70, |_, words| {
            assert!(words.is_empty());
            Ret {
                eax: 0x4444,
                ..Ret::default()
            }
        });
        e.register_double(0x00ad_7550, |_, words| {
            assert_eq!(words[0], 0x4444);
            assert_ne!(words[1], 0);
            assert_eq!(words[2..], [0x0107_37d0, 0x121]);
            Ret {
                eax: 0x5555,
                ..Ret::default()
            }
        });
        e.register_double(0x00ad_8830, |_, words| {
            assert_eq!(words, &[0x5555, 0]);
            Ret::default()
        });
        e.register_double(0x0048_3710, |_, _| Ret::default());
        e.register_double(0x00a0_12d0, move |_, words| {
            assert_eq!(words, &[popup, 0xfa9, 0, 1]);
            Ret::default()
        });
        e.register_double(0x00a0_7c60, move |_, words| {
            assert_eq!(
                words,
                &[popup, 0xfa9, 0, 255.0_f32.to_bits(), 0.5_f32.to_bits(), 0]
            );
            Ret::default()
        });
        e.register_double(0x0040_3df0, |_, words| {
            assert_eq!(words, &[0x011d_4324]);
            Ret {
                eax: 0x6666,
                ..Ret::default()
            }
        });
        e.register_double(0x00a0_1350, move |_, words| {
            assert_eq!(words, &[popup, 0xfc4, 0x6666, 1]);
            Ret::default()
        });
        e.register_double(0x0070_0320, move |_, words| {
            assert!(words == [popup, 0xfa3, 1] || words == [popup, 0x100e, 1]);
            Ret::default()
        });
        e.register_double(0x0082_5c00, |_, words| {
            assert_eq!(words, &[0x011f_6394]);
            Ret {
                eax: 250,
                ..Ret::default()
            }
        });
        e.register_double(0x0071_5c60, move |_, words| {
            assert_eq!(words, &[popup, 0x100d, 250]);
            Ret::default()
        });
        hud_main_menu_update_xp_meter(&mut e, menu);

        // 0077d928 / 0077d944: with the level-up deferred and hidden (flags 4 | 8), the
        // finished and zero-alpha reads take the popup tile at this+0xf0, not the meter.
        e.mem.set_u32(0x011a_010c, 0xc);
        e.mem.set_u8(menu.addr() + 0x204, 0);
        e.register_double(0x00a0_11b0, move |_, words| {
            assert!(words == [meter, 0xfa9] || words == [popup, 0xfa9]);
            Ret {
                st0: if words[0] == meter { 1.0 } else { 0.0 },
                ..Ret::default()
            }
        });
        e.register_double(0x00a0_7fc0, move |_, words| {
            assert!(words == [meter, 0xfa9] || words == [popup, 0xfa9]);
            Ret {
                eax: u32::from(words[0] == popup),
                ..Ret::default()
            }
        });
        e.register_double(0x0070_0320, move |_, words| {
            assert_eq!(words, &[popup, 0x100e, 0]);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        hud_main_menu_update_xp_meter(&mut e, menu);
        assert_eq!(e.mem.u8(menu.addr() + 0x204), 1);
        assert_eq!(e.mem.u32(0x011a_010c), 8);
        let calls = e.call_log.take().unwrap();
        assert_eq!(
            calls.last().unwrap(),
            &(0x0070_0320, args![popup, 0x100e_u32, 0_u32].to_vec())
        );
    }

    #[test]
    fn xp_gain_start_interpolates_previous_total_then_populates_six_tiles() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        e.map(0x011d_e000, 0x2000);
        e.map(0x0101_0000, 0x10000);
        e.mem.set_u32(0x011d_9e84, 0x1111);
        e.mem.set_u32(0x011d_9e80, 0x2222);
        e.mem.set_f32(0x011d_979c, 255.0);
        e.mem.set_f32(0x0101_62c0, 2.0);
        let menu = e.new_object::<HUDMainMenu>();
        let mut tiles = [0_u32; 6];
        for (index, offset) in [0xd8, 0xdc, 0xe0, 0xe4, 0xe8, 0xec].into_iter().enumerate() {
            tiles[index] = e.mem.alloc(4);
            e.mem.set_u32(menu.addr() + offset, tiles[index]);
        }
        let player = e.mem.alloc(0xa8);
        e.mem.set_u32(0x011d_ea3c, player);
        e.put_vtable(0x0300_7000, &[0, 0, 0, 0x0300_7010]);
        e.mem.set_u32(player + 0xa4, 0x0300_7000);
        e.register_double(0x0041_a250, |_, _| Ret::default());
        e.register_double(0x0045_3a70, |_, words| {
            assert!(words.is_empty());
            Ret {
                eax: 0x4444,
                ..Ret::default()
            }
        });
        e.register_double(0x00ad_7550, |_, words| {
            assert_eq!(words[0], 0x4444);
            assert_ne!(words[1], 0);
            assert_eq!(words[2..], [0x0107_37e0, 0x121]);
            Ret {
                eax: 0x5555,
                ..Ret::default()
            }
        });
        e.register_double(0x0041_8900, |_, words| {
            assert_eq!(words[1], 0x5555);
            Ret::default()
        });
        e.register_double(0x0048_3710, |_, _| Ret::default());
        e.register_double(0x00ad_8830, |_, words| {
            assert_eq!(words[1], 0);
            Ret::default()
        });
        e.register_double(0x0300_7010, move |_, words| {
            assert_eq!(words, &[player + 0xa4, 0x18]);
            Ret {
                st0: 150.0,
                ..Ret::default()
            }
        });
        e.register_double(0x00ec_62c0, |_, _| Ret {
            eax: 150,
            ..Ret::default()
        });
        e.register_double(0x0087_f9f0, move |_, words| {
            assert_eq!(words, &[player]);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.register_double(0x0064_8b50, |_, words| Ret {
            eax: match words {
                [1] => 100,
                [2] => 200,
                _ => panic!("unexpected required XP level"),
            },
            ..Ret::default()
        });
        e.register_double(0x00a0_11b0, move |_, words| {
            assert_eq!(words[0], tiles[3]);
            Ret {
                st0: match words[1] {
                    0x1111 => 0.0,
                    0x2222 => 1.0,
                    _ => panic!("unexpected XP meter trait"),
                },
                ..Ret::default()
            }
        });
        e.register_double(0x004b_3ab0, |_, words| {
            assert_eq!(
                words,
                &[
                    0.0_f32.to_bits(),
                    1.0_f32.to_bits(),
                    100.0_f32.to_bits(),
                    200.0_f32.to_bits(),
                    130.0_f32.to_bits()
                ]
            );
            Ret {
                st0: 0.4,
                ..Ret::default()
            }
        });
        e.register_double(0x0040_4010, |_, words| {
            assert_eq!(words, &[0.4_f32.to_bits(), 0]);
            Ret {
                st0: 0.4,
                ..Ret::default()
            }
        });
        e.register_double(0x0040_ebd0, |_, words| {
            assert_eq!(words, &[0.4_f32.to_bits(), 1.0_f32.to_bits()]);
            Ret {
                st0: 0.4,
                ..Ret::default()
            }
        });
        e.register_double(0x00a0_12d0, move |_, words| {
            if words[0] == tiles[3] && words[1] == 0xfa1 {
                assert_eq!(words[2..], [0.4_f32.to_bits(), 1]);
            } else {
                assert!(tiles.contains(&words[0]));
                assert_eq!(words[1..], [0xfa9, 0, 1]);
            }
            Ret::default()
        });
        e.register_double(0x0070_0320, move |_, words| {
            assert!(tiles.contains(&words[0]));
            assert_eq!(words[1..], [0xfa3, 1]);
            Ret::default()
        });
        e.register_double(0x0040_3df0, |_, words| {
            assert_eq!(words, &[0x011d_3bd4]);
            Ret {
                eax: 0x6666,
                ..Ret::default()
            }
        });
        e.register_double(0x00a0_1350, move |e, words| {
            assert!(tiles.contains(&words[0]));
            assert_eq!(words[1], 0xfc4);
            assert_eq!(words[3], 1);
            if words[0] == tiles[1] {
                e.mem.set_u32(0x011d_9e7c, 7);
            } else if words[0] == tiles[4] {
                e.mem.set_u32(0x011d_9e78, 8);
            }
            Ret::default()
        });
        e.register_double(0x0040_6d00, |_, words| {
            assert_eq!(words[1], 16);
            assert!(words[2] == 0x0107_34b0 || words[2] == 0x0102_0774);
            Ret::default()
        });
        e.register_double(0x00a0_7c60, move |_, words| {
            assert!(tiles.contains(&words[0]));
            assert_eq!(
                words[1..],
                [0xfa9, 0, 255.0_f32.to_bits(), 2.0_f32.to_bits(), 0]
            );
            Ret::default()
        });
        e.call_log = Some(vec![]);
        xp_start_gain(&mut e, menu, 20, false);
        let formatted: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == 0x0040_6d00)
            .map(|(_, words)| words[3])
            .collect();
        assert_eq!(formatted, vec![20, 7, 8]);
    }

    #[test]
    fn xp_animation_fades_all_six_tiles_after_position_and_meter_finish() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        e.map(0x0101_0000, 0x10000);
        e.mem.set_u32(0x011d_9e84, 0x1111);
        e.mem.set_u32(0x011d_9e80, 0x2222);
        e.mem.set_f64(0x0101_2060, 0.0);
        e.mem.set_f32(0x0101_62c0, 2.0);
        let menu = e.new_object::<HUDMainMenu>();
        let mut tiles = [0_u32; 6];
        for (index, offset) in [0xd8, 0xdc, 0xe0, 0xe4, 0xe8, 0xec].into_iter().enumerate() {
            tiles[index] = e.mem.alloc(4);
            e.mem.set_u32(menu.addr() + offset, tiles[index]);
        }
        e.register_double(0x00a0_7f30, move |_, words| {
            assert_eq!(words, &[tiles[3], 0xfa1]);
            Ret::default()
        });
        e.register_double(0x00a0_11b0, move |e, words| {
            if words == [tiles[0], 0xfa9] {
                let reads = e.mem.u32(0x011d_9e90);
                e.mem.set_u32(0x011d_9e90, reads + 1);
                if reads == 1 {
                    e.mem.set_f32(0x0101_62c0, 3.0);
                }
            }
            Ret {
                st0: if words == [tiles[3], 0x1111] {
                    0.0
                } else if words == [tiles[3], 0x2222] {
                    1.0
                } else if words[1] == 0xfa9 && tiles.contains(&words[0]) {
                    80.0
                } else {
                    panic!("unexpected XP animation trait")
                },
                ..Ret::default()
            }
        });
        e.register_double(0x00a0_7fc0, move |_, words| {
            assert!(words == [tiles[3], 0xfa1] || words == [tiles[0], 0xfa9]);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.register_double(0x00a0_1230, move |_, words| {
            assert_eq!(words, &[tiles[3], 0x100f]);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.register_double(0x0070_0320, move |_, words| {
            assert_eq!(words, &[tiles[3], 0x100f, 0]);
            Ret::default()
        });
        e.register_double(0x00a0_7c60, move |_, words| {
            assert!(tiles.contains(&words[0]));
            let duration = if words[0] == tiles[0] {
                2.0_f32
            } else {
                3.0_f32
            };
            assert_eq!(
                words[1..],
                [0xfa9, 80.0_f32.to_bits(), 0, duration.to_bits(), 0]
            );
            Ret::default()
        });
        e.call_log = Some(vec![]);
        xp_animate_current(&mut e, menu, true, true, false, false);
        assert_eq!(
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|(address, _)| *address == 0x00a0_7c60)
                .count(),
            6
        );
    }

    #[test]
    fn clear_xp_meter_resets_shared_state_with_or_without_hud() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        e.map(0x011a_0000, 0x2000);
        e.mem.set_u32(0x011a_010c, 8);
        hud_main_menu_clear_xp_meter_finished_moving(&mut e);
        assert_eq!(e.mem.u32(0x011a_010c), 1);
        let menu = e.new_object::<HUDMainMenu>();
        e.mem.set_u8(menu.addr() + 0x204, 1);
        e.mem.set_u32(0x011d_96c0, menu.addr());
        e.mem.set_u32(0x011a_010c, 4);
        hud_main_menu_clear_xp_meter_finished_moving(&mut e);
        assert_eq!(e.mem.u8(menu.addr() + 0x204), 0);
        assert_eq!(e.mem.u32(0x011a_010c), 1);
    }

    #[test]
    fn hotkey_wheel_keyboard_press_sets_highlight_and_empty_item_icon() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x6000);
        let hud = e.new_object::<HUDMainMenu>();
        let wheel = hud.addr() + 0x1c8;
        let label = e.mem.alloc(4);
        let icon = e.mem.alloc(4);
        e.mem.set_u32(hud.addr() + 0xcc, label);
        e.mem.set_u32(hud.addr() + 0xc8, icon);
        e.map(0x5000, 0x1000);
        e.mem.set_u32(0x011d_ea3c, 0x5000);
        e.mem.set_u32(0x011d_ea0c, 0x6000);
        e.register_double(0x004b_7210, |_, words| {
            assert!(words.is_empty());
            Ret {
                eax: 0x7000,
                ..Ret::default()
            }
        });
        e.register_double(0x0070_23a0, |_, words| {
            assert_eq!(words, &[0x7000]);
            Ret::default()
        });
        let mut player_slots = [0_u32; 0x8c];
        player_slots[0x22c / 4] = 0x0200_0000;
        e.put_vtable(0x0300_5000, &player_slots);
        e.mem.set_u32(0x5000, 0x0300_5000);
        e.register_double(0x0200_0000, |_, words| {
            assert_eq!(words, &[0x5000, 0]);
            Ret::default()
        });
        e.register_double(0x005a_03f0, |_, words| {
            assert_eq!(words, &[0x5000, 4]);
            Ret::default()
        });
        e.register_double(0x004b_71d0, |_, words| {
            assert!(words.is_empty());
            Ret::default()
        });
        e.register_double(0x006e_4ba0, move |_, words| {
            assert_eq!(words, &[wheel]);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.register_double(0x0087_7720, |_, words| {
            assert_eq!(words, &[0x6000]);
            Ret {
                eax: 0x7111,
                ..Ret::default()
            }
        });
        e.register_double(0x00a2_4660, |_, words| {
            assert_eq!(words, &[0x7111, 0x11, 1]);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.register_double(0x004b_f220, |_, words| {
            assert_eq!(words, &[0x5000]);
            Ret::default()
        });
        e.register_double(0x0082_5c00, |_, words| {
            assert_eq!(words, &[0x011f_6394]);
            Ret {
                eax: 1234,
                ..Ret::default()
            }
        });
        e.register_double(0x0070_1bd0, move |_, words| {
            assert_eq!(words, &[wheel, 0]);
            Ret::default()
        });
        e.register_double(0x00a0_1350, move |_, words| {
            assert_eq!(words, &[label, 0xfc4, 0x0102_0770, 1]);
            Ret::default()
        });
        e.register_double(0x00a0_1860, |_, words| {
            assert_eq!(words, &[0x0107_3814]);
            Ret {
                eax: 0xffe,
                ..Ret::default()
            }
        });
        e.register_double(0x0070_0320, move |_, words| {
            assert_eq!(words, &[icon, 0xffe, 0]);
            Ret::default()
        });
        e.register_double(0x0070_1740, move |_, words| {
            assert_eq!(words, &[wheel]);
            Ret::default()
        });
        hud_main_menu_update_hotkeys_wheel(&mut e, hud);
        assert_eq!(e.mem.u32(0x011d_9e8c), 1234);
    }

    #[test]
    fn hotkey_wheel_blocked_gate_hides_wheel_before_player_access() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x6000);
        let hud = e.new_object::<HUDMainMenu>();
        let label = e.mem.alloc(4);
        e.mem.set_u32(hud.addr() + 0xcc, label);
        e.mem.set_u32(0x011d_9e8c, 99);
        e.register_double(0x004b_7210, |_, words| {
            assert!(words.is_empty());
            Ret {
                eax: 0x7000,
                ..Ret::default()
            }
        });
        e.register_double(0x0070_23a0, |_, words| {
            assert_eq!(words, &[0x7000]);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        let wheel = hud.addr() + 0x1c8;
        e.register_double(0x0070_1760, move |_, words| {
            assert_eq!(words, &[wheel, 0]);
            Ret::default()
        });
        e.register_double(0x0070_0320, move |_, words| {
            assert_eq!(words, &[label, 0xfa3, 0]);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        hud_main_menu_update_hotkeys_wheel(&mut e, hud);
        assert_eq!(e.mem.u32(0x011d_9e8c), 0);
        assert_eq!(e.call_log.as_ref().unwrap().len(), 4);
    }

    #[test]
    fn hud_indexed_alpha_skips_pc_switch_entries_and_reloads_scale_after_clear() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        e.map(0x0101_e000, 0x1000);
        e.mem.set_f64(0x0101_e568, 255.0);
        let hud = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, hud.addr());
        for index in 1..0x3b_u32 {
            e.mem.set_u32(hud.addr() + index * 4 + 0x28, 0x8000 + index);
        }
        e.register_double(0x0077_5920, move |e, words| {
            assert_eq!(words, &[hud.addr()]);
            e.mem.set_f64(0x0101_e568, 2.0);
            Ret::default()
        });
        e.register_double(0x00a0_12d0, |_, words| {
            assert_eq!(words[1..], [0xfa9, 1.0_f32.to_bits(), 1]);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        fn_0077e430(&mut e, 0.5);
        let indices: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == 0x00a0_12d0)
            .map(|(_, words)| words[0] - 0x8000)
            .collect();
        assert_eq!(indices.len(), 39);
        assert!(indices.contains(&1) && indices.contains(&0xe));
        assert!(!indices.contains(&0xf) && !indices.contains(&0x14) && !indices.contains(&0x3a));
        assert!(indices.contains(&0x15) && indices.contains(&0x33));
    }

    #[test]
    fn breath_indicator_fades_in_when_breath_is_full_and_alpha_zero() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x6000);
        e.map(0x0101_0000, 0x10000);
        e.map(0x0107_3000, 0x1000);
        e.mem.set_f64(0x0101_6408, 0.01);
        e.mem.set_f64(0x0107_3828, 0.875);
        e.mem.set_f64(0x0101_7a40, 100.0);
        e.mem.set_f32(0x0101_6248, 0.5);
        e.mem.set_f32(0x011d_979c, 255.0);
        let hud = e.new_object::<HUDMainMenu>();
        let tile = e.mem.alloc(4);
        e.mem.set_u32(hud.addr() + 0x148, tile);
        let player = e.mem.alloc(4);
        let state = e.mem.alloc(12);
        e.mem.set_f32(state + 8, 1.0);
        e.mem.set_u32(0x011d_ea3c, player);
        let mut slots = [0_u32; 0x7e];
        slots[0x1f4 / 4] = 0x0300_21f4;
        e.put_vtable(0x0300_2000, &slots);
        e.mem.set_u32(player, 0x0300_2000);
        e.register_double(0x008d_6f30, move |_, words| {
            assert_eq!(words, &[player]);
            Ret {
                eax: 0x9000,
                ..Ret::default()
            }
        });
        e.register_double(0x0300_21f4, move |_, words| {
            assert_eq!(words, &[player]);
            Ret {
                eax: state,
                ..Ret::default()
            }
        });
        e.register_double(0x0088_5560, move |_, words| {
            assert_eq!(words, &[player, 1.0_f32.to_bits(), 0x9000]);
            Ret {
                st0: 0.9,
                ..Ret::default()
            }
        });
        e.register_double(0x008b_e7a0, move |_, words| {
            assert_eq!(words, &[player]);
            Ret {
                st0: 5.0,
                ..Ret::default()
            }
        });
        e.register_double(0x00ec_62c0, |_, words| {
            assert_eq!(words, &args![5.0_f64]);
            Ret {
                eax: 5,
                ..Ret::default()
            }
        });
        e.register_double(0x0064_8a10, |_, words| {
            assert_eq!(words, &[5]);
            Ret {
                st0: 10.0,
                ..Ret::default()
            }
        });
        e.register_double(0x008d_8520, move |_, words| {
            assert_eq!(words, &[player]);
            Ret::default()
        });
        e.register_double(0x00a0_11b0, move |_, words| {
            assert_eq!(words, &[tile, 0xfa9]);
            Ret::default()
        });
        e.register_double(0x00a0_7c60, move |_, words| {
            assert_eq!(
                words,
                &[tile, 0xfa9, 0, 255.0_f32.to_bits(), 0.5_f32.to_bits(), 0]
            );
            Ret::default()
        });
        fn_0077e4f0(&mut e, hud);
    }

    #[test]
    fn target_indicator_fades_unused_tile_with_precallback_duration() {
        let mut e = Engine::new();
        e.map(0x0101_0000, 0x10000);
        e.mem.set_f64(0x0101_2060, 0.0);
        e.mem.set_f32(0x0101_6248, 2.0);
        let hud = e.new_object::<HUDMainMenu>();
        let indicators = e.mem.alloc(8);
        let node = e.mem.alloc(8);
        let slot = e.mem.alloc(4);
        let tile = e.mem.alloc(4);
        e.mem.set_u32(slot, tile);
        e.mem.set_u32(hud.addr() + 0x14c, indicators);
        e.register_double(0x0055_9450, move |_, words| {
            assert_eq!(words, &[indicators + 4]);
            Ret {
                eax: node,
                ..Ret::default()
            }
        });
        e.register_double(0x0071_7e50, |_, words| {
            assert_eq!(words, &[0x011e_0e80]);
            Ret {
                eax: 0x9000,
                ..Ret::default()
            }
        });
        e.register_double(0x005d_e0f0, |_, words| {
            assert_eq!(words, &[0x9000, 0]);
            Ret::default()
        });
        e.register_double(0x005b_e5c0, |_, words| {
            assert_eq!(words, &[0x9000, 0]);
            Ret::default()
        });
        e.register_double(0x0057_cbe0, move |e, words| {
            assert_eq!(words[0], indicators + 4);
            assert_eq!(e.mem.u32(words[1]), node);
            e.mem.set_u32(words[1], 0);
            Ret {
                eax: slot,
                ..Ret::default()
            }
        });
        e.register_double(0x00a0_7fc0, move |_, words| {
            assert_eq!(words, &[tile, 0xfa9]);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.register_double(0x00a0_11b0, move |e, words| {
            assert_eq!(words, &[tile, 0xfa9]);
            let count = e.mem.u32(indicators);
            e.mem.set_u32(indicators, count + 1);
            if count == 1 {
                e.mem.set_f32(0x0101_6248, 3.0);
            }
            Ret {
                st0: 80.0,
                ..Ret::default()
            }
        });
        e.register_double(0x00a0_7c60, move |_, words| {
            assert_eq!(
                words,
                &[tile, 0xfa9, 80.0_f32.to_bits(), 0, 2.0_f32.to_bits(), 0]
            );
            Ret::default()
        });
        fn_0077e750(&mut e, hud);
        assert_eq!(e.mem.u32(indicators), 2);
    }

    #[test]
    fn target_indicator_accepts_full_eax_process_flag() {
        let mut e = Engine::new();
        e.map(0x0101_2000, 0x1000);
        e.mem.set_f64(0x0101_2060, 0.0);
        let hud = e.new_object::<HUDMainMenu>();
        let indicators = e.mem.alloc(8);
        e.mem.set_u32(hud.addr() + 0x14c, indicators);
        let actor = e.mem.alloc(4);
        let mut slots = [0_u32; 0x8a];
        slots[0x224 / 4] = 0x0300_8224;
        e.put_vtable(0x0300_8000, &slots);
        e.mem.set_u32(actor, 0x0300_8000);
        e.register_double(0x0055_9450, move |_, words| {
            assert_eq!(words, &[indicators + 4]);
            Ret::default()
        });
        e.register_double(0x0071_7e50, |_, words| {
            assert_eq!(words, &[0x011e_0e80]);
            Ret {
                eax: 0x9000,
                ..Ret::default()
            }
        });
        e.register_double(0x005d_e0f0, |_, words| {
            assert_eq!(words, &[0x9000, 0]);
            Ret::default()
        });
        e.register_double(0x005b_e5c0, |_, words| {
            assert_eq!(words, &[0x9000, 0]);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.register_double(0x0096_8670, move |_, words| {
            assert_eq!(words, &[0x9000, 0]);
            Ret {
                eax: actor,
                ..Ret::default()
            }
        });
        for address in [0x0057_6d30, 0x0044_0d80, 0x0044_0da0] {
            e.register_double(address, move |_, words| {
                assert_eq!(words, &[actor]);
                Ret::default()
            });
        }
        e.register_double(0x0040_1170, move |_, words| {
            assert_eq!(words, &[actor]);
            Ret {
                eax: 0x3e,
                ..Ret::default()
            }
        });
        e.register_double(0x0300_8224, move |_, words| {
            assert_eq!(words, &[actor]);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.register_double(0x0077_eac0, move |_, words| {
            assert_eq!(words, &[actor]);
            Ret::default()
        });
        e.register_double(0x0041_81e0, move |_, words| {
            assert_eq!(words, &[actor]);
            Ret {
                eax: 0xa000,
                ..Ret::default()
            }
        });
        e.register_double(0x004f_d3c0, |_, words| {
            assert_eq!(words, &[0xa000]);
            Ret {
                eax: 0x100,
                ..Ret::default()
            }
        });
        e.register_double(0x0045_cd80, |_, words| {
            assert_eq!(words, &[0xa000]);
            Ret {
                st0: 1.0,
                ..Ret::default()
            }
        });
        e.call_log = Some(vec![]);
        fn_0077e750(&mut e, hud);
        assert_eq!(
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|(a, _)| *a == 0x0045_cd80)
                .count(),
            1
        );
    }

    #[test]
    fn target_registry_helper_negates_membership_result() {
        let mut e = Engine::new();
        e.map(0x011c_3000, 0x1000);
        e.mem.set_u32(0x011c_3f2c, 0x4444);
        e.register_double(0x0084_e3a0, |_, words| {
            assert_eq!(words, &[0x2000]);
            Ret {
                eax: 0x1234,
                ..Ret::default()
            }
        });
        e.register_double(0x0046_9860, |_, words| {
            assert_eq!(words, &[0x4444, 0x1234]);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        assert!(!fn_0077eac0(&mut e, Ptr::new(0x2000)));
        e.register_double(0x0046_9860, |_, _| Ret::default());
        assert!(fn_0077eac0(&mut e, Ptr::new(0x2000)));
    }

    #[test]
    fn crippled_guy_uses_face_ratio_and_six_limb_states() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        e.map(0x011d_e000, 0x1000);
        e.map(0x011a_0000, 0x1000);
        e.map(0x0101_7000, 0x1000);
        e.map(0x0102_0000, 0x1000);
        e.mem.set_f64(0x0101_7a40, 100.0);
        e.mem.set_f64(0x0102_0998, 5.0);
        e.mem.set_f32(0x0101_712c, 5.0);
        e.mem.set_f32(0x011d_979c, 255.0);
        let hud = e.new_object::<HUDMainMenu>();
        let face_tile = e.mem.alloc(4);
        e.mem.set_u32(hud.addr() + 0xf8, face_tile);
        let mut limb_tiles = [0_u32; 6];
        for (n, index) in (0x19..0x1f_u32).enumerate() {
            limb_tiles[n] = e.mem.alloc(4);
            e.mem
                .set_u32(hud.addr() + 0x28 + (index + 0x1c) * 4, limb_tiles[n]);
            e.mem.set_u32(0x011a_00c4 + index * 4, 0xa000 + index);
        }
        let player = e.mem.alloc(0xa8);
        e.mem.set_u32(0x011d_ea3c, player);
        let mut list_slots = [0_u32; 3];
        list_slots[2] = 0x0300_9408;
        e.put_vtable(0x0300_9400, &list_slots);
        e.mem.set_u32(player + 0x94, 0x0300_9400);
        let mut value_slots = [0_u32; 9];
        value_slots[3] = 0x0300_a40c;
        value_slots[8] = 0x0300_a420;
        e.put_vtable(0x0300_a400, &value_slots);
        e.mem.set_u32(player + 0xa4, 0x0300_a400);
        e.register_double(0x0300_9408, move |_, words| {
            assert_eq!(words, &[player + 0x94]);
            Ret::default()
        });
        e.register_double(0x0300_a420, move |_, words| {
            assert_eq!(words, &[player + 0xa4, 0x10]);
            Ret {
                st0: 100.0,
                ..Ret::default()
            }
        });
        e.register_double(0x0300_a40c, move |_, words| {
            assert_eq!(words[0], player + 0xa4);
            Ret {
                st0: if words[1] == 0x10 {
                    80.0
                } else if words[1] == 0x19 {
                    0.0
                } else {
                    100.0
                },
                ..Ret::default()
            }
        });
        e.register_double(0x00ec_62c0, |_, words| {
            assert_eq!(words.len(), 2);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.register_double(0x0040_6d00, |_, words| {
            assert_eq!(words[1], 0x104);
            if words[2] == 0x0107_3868 {
                assert_eq!(words[3], 1);
            } else {
                assert!(words[2] == 0x0107_3830 || words[2] == 0x0107_3850);
            }
            Ret::default()
        });
        e.register_double(0x00a0_1350, |_, words| {
            assert_eq!(words[1], 0xfcc);
            assert_eq!(words[3], 1);
            Ret::default()
        });
        e.register_double(0x0070_0320, move |_, words| {
            assert!(limb_tiles.contains(&words[0]));
            assert_eq!(words[1], 0xff4);
            assert_eq!(words[2], if words[0] == limb_tiles[0] { 2 } else { 1 });
            Ret::default()
        });
        e.register_double(0x00a0_7c60, move |_, words| {
            assert_eq!(
                words[1..5],
                [0xfa9, 0, 255.0_f32.to_bits(), 5.0_f32.to_bits()]
            );
            assert_eq!(words[5], if words[0] == limb_tiles[0] { 3 } else { 4 });
            Ret::default()
        });
        e.call_log = Some(vec![]);
        hud_main_menu_show_crippled_guy(&mut e, hud);
        assert_eq!(
            e.call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|(a, _)| *a == 0x00a0_1350)
                .count(),
            7
        );
    }

    #[test]
    fn hud_camera_uses_override_transform_and_renderer_aspect() {
        let mut e = Engine::new();
        e.map(0x011f_9000, 0x1000);
        e.map(0x011a_d000, 0x1000);
        e.map(0x0101_6000, 0x1000);
        e.map(0x0102_3000, 0x1000);
        e.map(0x0107_0000, 0x1000);
        e.mem.set_u8(0x011f_9426, 1);
        e.mem.set_i32(0x011f_9434, 1920);
        for (offset, value) in [(0, 1_u32), (4, 2), (8, 3), (12, 4)] {
            e.mem.set_u32(0x011a_d840 + offset, value);
        }
        e.mem.set_f32(0x0101_6264, 0.75);
        e.mem.set_f64(0x0102_3128, 0.017_453_292);
        e.mem.set_f64(0x0107_0c80, 0.15);
        let ratio_setting = e.mem.alloc(4);
        let angle_setting = e.mem.alloc(4);
        e.mem.set_f32(ratio_setting, 2.0);
        e.mem.set_f32(angle_setting, 1.0);
        let view = e.mem.alloc(4);
        let mut slots = [0_u32; 0x41];
        slots[0x100 / 4] = 0x0300_6100;
        e.put_vtable(0x0300_6000, &slots);
        e.mem.set_u32(view, 0x0300_6000);
        e.register_double(0x00aa_13e0, |_, words| {
            assert_eq!(words, &[0x114]);
            Ret {
                eax: 0x5000,
                ..Ret::default()
            }
        });
        e.register_double(0x00a7_12f0, |_, words| {
            assert_eq!(words, &[0x5000]);
            Ret {
                eax: 0x6000,
                ..Ret::default()
            }
        });
        e.register_double(0x0041_4430, |_, words| {
            assert_eq!(words[1..], [0, 1.0_f32.to_bits(), 1.0_f32.to_bits(), 0]);
            Ret {
                eax: words[0],
                ..Ret::default()
            }
        });
        e.register_double(0x0071_2e60, |e, words| {
            assert_eq!(words[0], 0x6000);
            assert_eq!(
                (0..4)
                    .map(|i| e.mem.u32(words[1] + i * 4))
                    .collect::<Vec<_>>(),
                [1, 2, 3, 4]
            );
            Ret::default()
        });
        e.register_double(0x00a7_1b70, |_, words| {
            assert_eq!(words[1], 0);
            Ret::default()
        });
        e.register_double(0x0045_c670, move |_, words| {
            assert!(words.is_empty());
            Ret {
                eax: view,
                ..Ret::default()
            }
        });
        e.register_double(0x0300_6100, move |_, words| {
            assert_eq!(words, &[view]);
            Ret {
                st0: 1.0,
                ..Ret::default()
            }
        });
        e.register_double(0x0040_3e20, move |_, words| Ret {
            eax: match words {
                [0x0120_3144] => ratio_setting,
                [0x0120_3150] => angle_setting,
                _ => panic!("unexpected HUD camera setting"),
            },
            ..Ret::default()
        });
        e.register_double(0x0050_7700, |_, words| {
            assert_eq!(words, &[0x6000, 0.5_f32.to_bits()]);
            Ret::default()
        });
        e.register_double(0x004d_c1f0, |_, words| {
            assert!(words.is_empty());
            Ret {
                eax: 1080,
                ..Ret::default()
            }
        });
        e.register_double(0x0057_dd70, |_, words| {
            assert_eq!(words.len(), 1);
            Ret {
                st0: 0.5,
                ..Ret::default()
            }
        });
        e.register_double(0x00a6_faf0, |e, words| {
            assert_eq!(words[0], 0x6000);
            let frustum = words[1];
            assert_eq!(e.mem.f32(frustum), -0.375);
            assert_eq!(e.mem.f32(frustum + 4), 0.375);
            assert!((e.mem.f32(frustum + 8) - 0.666_666_7).abs() < 0.000_001);
            assert!((e.mem.f32(frustum + 12) + 0.666_666_7).abs() < 0.000_001);
            assert_eq!(e.mem.f32(frustum + 0x10), 2.0);
            assert_eq!(e.mem.f32(frustum + 0x14), 1.0);
            assert_eq!(e.mem.u8(frustum + 0x18), 0);
            Ret::default()
        });
        e.register_double(0x0041_6870, |_, words| Ret {
            eax: words[0],
            ..Ret::default()
        });
        e.register_double(0x00a7_01b0, |_, words| {
            assert_eq!(words[0], 0x6000);
            assert_eq!(words.len(), 3);
            Ret::default()
        });
        e.register_double(0x0073_3fd0, |_, words| {
            assert_eq!(words, &[0x6000, 1.0_f32.to_bits()]);
            Ret::default()
        });
        e.register_double(0x0043_d410, |_, words| {
            assert_eq!(words[1..], [0, 0, 0]);
            Ret::default()
        });
        e.register_double(0x00a5_9c60, |_, words| {
            assert_eq!(words[0], 0x6000);
            assert_eq!(words.len(), 2);
            Ret::default()
        });
        assert_eq!(
            hud_main_menu_create_3d_camera(&mut e, Ptr::NULL).addr(),
            0x6000
        );
    }

    #[test]
    fn hud_scene_render_restores_dialogue_and_destroys_token() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        e.map(0x011a_0000, 0x1000);
        e.mem.set_f32(0x011a_0140, 0.75);
        e.register_double(0x0055_9450, |_, words| Ret {
            eax: match words {
                [0x011d_9770] => 0x7000,
                [0x011d_975c] => 0xe000,
                [_] => 0xf000,
                _ => panic!("unexpected 3D scene getter"),
            },
            ..Ret::default()
        });
        e.register_double(0x0045_0b80, |_, words| {
            assert_eq!(words, &[1]);
            Ret {
                eax: 0x8000,
                ..Ret::default()
            }
        });
        e.register_double(0x0045_3470, |_, words| {
            assert_eq!(words, &[0x8000]);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.register_double(0x0043_c4b0, |_, words| {
            assert!(words.is_empty());
            Ret {
                eax: 0x8100,
                ..Ret::default()
            }
        });
        e.register_double(0x0071_48c0, |_, words| {
            assert_eq!(words, &[0x8100, 0, 4]);
            Ret::default()
        });
        e.register_double(0x0045_c670, |_, words| {
            assert!(words.is_empty());
            Ret {
                eax: 0xa000,
                ..Ret::default()
            }
        });
        e.register_double(0x008d_80e0, |_, words| {
            assert_eq!(words, &[0xa000]);
            Ret {
                eax: 0xb000,
                ..Ret::default()
            }
        });
        e.register_double(0x00c4_f270, |_, words| {
            assert_eq!(words, &[0xb000, 1]);
            Ret::default()
        });
        e.register_double(0x0070_9d50, |_, words| {
            assert_eq!(words, &[0xe000, 0.75_f32.to_bits()]);
            Ret::default()
        });
        e.register_double(0x004b_7210, |_, words| {
            assert!(words.is_empty());
            Ret {
                eax: 0xc000,
                ..Ret::default()
            }
        });
        e.register_double(0x007f_d640, |_, words| {
            assert_eq!(words, &[0xc000]);
            Ret {
                eax: 0xd000,
                ..Ret::default()
            }
        });
        e.register_double(0x0063_3c90, |_, words| {
            assert_eq!(words[1], 0xd000);
            Ret::default()
        });
        e.register_double(0x004a_0fd0, |_, words| {
            assert_eq!(words[0], 0xb000);
            assert!(words[1] == 0xf000 || words[1] == 0);
            Ret::default()
        });
        e.register_double(0x00b6_bee0, |_, words| {
            assert_eq!(words, &[0xe000, 0x8000, 0xb000]);
            Ret::default()
        });
        e.register_double(0x00b6_c0d0, |_, words| {
            assert_eq!(words, &[0xe000, 0xf000, 0]);
            Ret::default()
        });
        e.register_double(0x0066_b0d0, |_, words| {
            assert_eq!(words[1], 0);
            Ret::default()
        });
        e.register_double(0x00c4_f2d0, |_, words| {
            assert_eq!(words, &[0xb000]);
            Ret::default()
        });
        e.register_double(0x0045_cec0, |_, words| {
            assert_eq!(words.len(), 1);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        fn_0077f0d0(&mut e);
        let calls = e.call_log.as_ref().unwrap();
        assert_eq!(calls.last().unwrap().0, 0x0045_cec0);
    }

    #[test]
    fn hud_camera_detach_uses_sequence_then_clears_visibility() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        let hud = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, hud.addr());
        e.mem.set_u8(hud.addr() + 0x1fc, 1);
        let camera = e.mem.alloc(4);
        let mut slots = [0_u32; 0x3b];
        slots[0xe8 / 4] = 0x0300_70e8;
        e.put_vtable(0x0300_7000, &slots);
        e.mem.set_u32(camera, 0x0300_7000);
        e.register_double(0x0055_9450, move |_, words| {
            assert_eq!(words, &[0x011d_9770]);
            Ret {
                eax: camera,
                ..Ret::default()
            }
        });
        e.register_double(0x0045_bc00, move |_, words| {
            assert_eq!(words, &[camera, 0]);
            Ret {
                eax: 0x8000,
                ..Ret::default()
            }
        });
        e.register_double(0x0043_b4a0, move |_, words| {
            assert_eq!(words, &[camera, 0]);
            Ret {
                eax: 0x9000,
                ..Ret::default()
            }
        });
        e.register_double(0x0300_70e8, move |_, words| {
            assert_eq!(words, &[camera, 0x9000]);
            Ret::default()
        });
        fn_0077f270(&mut e);
        assert_eq!(e.mem.u8(hud.addr() + 0x1fc), 0);
    }

    #[test]
    fn hud_camera_model_replacement_preserves_loader_argument_order() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        e.map(0x011c_3000, 0x1000);
        let hud = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, hud.addr());
        e.mem.set_u32(0x011c_3b3c, 0x8000);
        let camera = e.mem.alloc(4);
        let mut camera_slots = [0_u32; 0x3a];
        camera_slots[0xdc / 4] = 0x0300_70dc;
        e.put_vtable(0x0300_7000, &camera_slots);
        e.mem.set_u32(camera, 0x0300_7000);
        let resource = e.mem.alloc(4);
        let mut resource_slots = [0_u32; 6];
        resource_slots[0x14 / 4] = 0x0300_8014;
        e.put_vtable(0x0300_8000, &resource_slots);
        e.mem.set_u32(resource, 0x0300_8000);
        e.register_double(0x0055_9450, move |_, words| {
            assert_eq!(words, &[0x011d_9770]);
            Ret {
                eax: camera,
                ..Ret::default()
            }
        });
        e.register_double(0x0045_bc00, move |_, words| {
            assert_eq!(words, &[camera, 0]);
            Ret::default()
        });
        e.register_double(0x0048_cee0, move |_, words| {
            assert_eq!(words, &[resource]);
            Ret {
                eax: 1,
                ..Ret::default()
            }
        });
        e.register_double(0x0300_8014, move |_, words| {
            assert_eq!(words, &[resource]);
            Ret {
                eax: 0x9000,
                ..Ret::default()
            }
        });
        e.register_double(0x0044_7080, |_, words| {
            assert_eq!(words, &[0x8000, 0x9000, 0, 1, 0, 0, 0]);
            Ret {
                eax: 0xa000,
                ..Ret::default()
            }
        });
        e.register_double(0x0300_70dc, move |_, words| {
            assert_eq!(words, &[camera, 0xa000, 1]);
            Ret::default()
        });
        e.register_double(0x0045_0f90, move |_, words| {
            assert_eq!(words, &[camera, 1]);
            Ret::default()
        });
        e.register_double(0x0043_d410, |_, words| {
            assert_eq!(words[1..], [0, 0, 0]);
            Ret::default()
        });
        e.register_double(0x00a5_9c60, move |_, words| {
            assert_eq!(words[0], camera);
            assert_eq!(words.len(), 2);
            Ret::default()
        });
        fn_0077f2f0(&mut e, Ptr::new(resource));
    }

    #[test]
    fn gun_scope_visibility_inverts_camera_fade_flag() {
        for (visible, expected_fade) in [(0_u8, 1_u32), (1, 0)] {
            let mut e = Engine::new();
            e.map(0x011d_9000, 0x1000);
            let hud = e.new_object::<HUDMainMenu>();
            e.mem.set_u32(0x011d_96c0, hud.addr());
            e.register_double(0x0055_9450, |_, words| {
                assert_eq!(words, &[0x011d_9770]);
                Ret {
                    eax: 0x7000,
                    ..Ret::default()
                }
            });
            e.register_double(0x0045_bc00, |_, words| {
                assert_eq!(words, &[0x7000, 0]);
                Ret {
                    eax: 1,
                    ..Ret::default()
                }
            });
            e.register_double(0x0045_0f90, move |_, words| {
                assert_eq!(words, &[0x7000, expected_fade]);
                Ret::default()
            });
            hud_main_menu_set_gun_scope_visible(&mut e, visible);
            assert_eq!(e.mem.u8(hud.addr() + 0x1fc), visible);
        }
    }

    #[test]
    fn first_status_indicator_records_time_and_selects_alternate_tile() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        let hud = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, hud.addr());
        e.mem.set_u32(hud.addr() + 0x1bc, 1);
        e.mem.set_u32(hud.addr() + 0x154, 0x7000);
        e.mem.set_u32(hud.addr() + 0x170, 0x8000);
        e.register_double(0x0082_5c00, |_, words| {
            assert_eq!(words, &[0x011f_6394]);
            Ret {
                eax: 123,
                ..Ret::default()
            }
        });
        e.register_double(0x0070_0320, |_, words| {
            assert!(words == [0x7000, 0xfa3, 1] || words == [0x8000, 0xfa3, 1]);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        fn_0077f420(&mut e, 0);
        fn_0077f420(&mut e, 1);
        assert_eq!(e.mem.u32(hud.addr() + 0x24c), 123);
        let tiles: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == 0x0070_0320)
            .map(|(_, words)| words[0])
            .collect();
        assert_eq!(tiles, vec![0x7000, 0x8000]);
    }

    #[test]
    fn second_status_indicator_records_time_and_selects_alternate_tile() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        let hud = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, hud.addr());
        e.mem.set_u32(hud.addr() + 0x1bc, 8);
        e.mem.set_u32(hud.addr() + 0x158, 0x7000);
        e.mem.set_u32(hud.addr() + 0x174, 0x8000);
        e.register_double(0x0082_5c00, |_, words| {
            assert_eq!(words, &[0x011f_6394]);
            Ret {
                eax: 456,
                ..Ret::default()
            }
        });
        e.register_double(0x0070_0320, |_, words| {
            assert!(words == [0x7000, 0xfa3, 1] || words == [0x8000, 0xfa3, 1]);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        fn_0077f490(&mut e, 0);
        fn_0077f490(&mut e, 1);
        assert_eq!(e.mem.u32(hud.addr() + 0x250), 456);
        let tiles: Vec<_> = e
            .call_log
            .as_ref()
            .unwrap()
            .iter()
            .filter(|(address, _)| *address == 0x0070_0320)
            .map(|(_, words)| words[0])
            .collect();
        assert_eq!(tiles, vec![0x7000, 0x8000]);
    }

    #[test]
    fn kill_quest_name_hides_tiles_then_releases_queue_records() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        let hud = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, hud.addr());
        e.mem.set_u32(0x011d_96bc, 0x6000);
        let pool = e.mem.alloc(8);
        let node = e.mem.alloc(8);
        let slot = e.mem.alloc(4);
        let record_slot = e.mem.alloc(4);
        e.mem.set_u32(slot, 0x8000);
        e.mem.set_u32(record_slot, 0x9000);
        e.mem.set_u32(hud.addr() + 0x60, pool);
        e.register_double(0x0055_9450, move |_, words| {
            assert_eq!(words, &[pool + 4]);
            Ret {
                eax: node,
                ..Ret::default()
            }
        });
        e.register_double(0x0057_cbe0, move |e, words| {
            assert_eq!(words[0], pool + 4);
            assert_eq!(e.mem.u32(words[1]), node);
            e.mem.set_u32(words[1], 0);
            Ret {
                eax: slot,
                ..Ret::default()
            }
        });
        e.register_double(0x0070_0320, |_, words| {
            assert_eq!(words, &[0x8000, 0xfa3, 0]);
            Ret::default()
        });
        e.register_double(0x0068_15c0, move |_, words| {
            assert_eq!(words, &[0x011d_970c]);
            Ret {
                eax: record_slot,
                ..Ret::default()
            }
        });
        e.register_double(0x0040_1030, |_, words| {
            assert_eq!(words, &[0x9000]);
            Ret::default()
        });
        e.register_double(0x0063_f7b0, move |e, words| {
            assert_eq!(words, &[0x011d_970c]);
            e.mem.set_u32(record_slot, 0);
            Ret::default()
        });
        e.register_double(0x0047_0470, |_, words| {
            assert_eq!(words, &[0x011d_970c]);
            Ret::default()
        });
        e.call_log = Some(vec![]);
        hud_main_menu_kill_quest_name(&mut e, hud);
        let calls = e.call_log.as_ref().unwrap();
        assert_eq!(calls.last().unwrap().0, 0x0047_0470);
        assert_eq!(calls.iter().filter(|(a, _)| *a == 0x0040_1030).count(), 1);
    }

    #[test]
    fn fixed_quest_queue_clear_uses_pc_global_address() {
        let mut e = Engine::new();
        e.register_double(0x0047_0470, |_, words| {
            assert_eq!(words, &[0x011d_970c]);
            Ret::default()
        });
        fn_0077f5c0(&mut e, Ptr::NULL);
    }

    #[test]
    fn list_first_or_fallback_uses_the_count_branch() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x710);
        let slot = e.mem.alloc(4);
        e.mem.set_u32(object + 0x6f4, 0x1111);
        e.mem.set_u32(slot, 0x2222);
        e.register_double(0x0044_ddc0, move |e, words| {
            assert_eq!(words, &[object + 0x708]);
            Ret {
                eax: e.mem.u32(object + 0x700),
                ..Ret::default()
            }
        });
        e.register_double(0x0055_8dd0, move |_, words| {
            assert_eq!(words, &[object + 0x708, 0]);
            Ret {
                eax: slot,
                ..Ret::default()
            }
        });
        assert_eq!(fn_0077a400(&mut e, Ptr::new(object)).addr(), 0x1111);
        e.mem.set_u32(object + 0x700, 1);
        assert_eq!(fn_0077a400(&mut e, Ptr::new(object)).addr(), 0x2222);
    }

    #[test]
    fn coordinate_pair_stores_float_words() {
        let mut e = Engine::new();
        let object = e.mem.alloc(0x84);
        fn_0077a450(&mut e, Ptr::new(object), -0.25, 1.5);
        assert_eq!(e.mem.u32(object + 0x7c), (-0.25_f32).to_bits());
        assert_eq!(e.mem.u32(object + 0x80), 1.5_f32.to_bits());
    }

    #[test]
    fn quest_name_queue_sets_active_quest_by_flags() {
        for (flags, completed, active, expected_state, expected_active) in [
            (0_u8, false, 0_u32, 0_u32, Some(0x1234_u32)),
            (0, true, 0x1234, 1, Some(0)),
            (0x40, false, 0x1234, 2, Some(0)),
        ] {
            let mut e = Engine::new();
            e.map(0x011d_9000, 0x6000);
            let hud = e.new_object::<HUDMainMenu>();
            let flags_ptr = e.mem.alloc(1);
            e.mem.set_u8(flags_ptr, flags);
            e.mem.set_u32(0x011d_96c0, hud.addr());
            e.mem.set_u32(0x011d_ea3c, 0x7777);
            e.register_double(0x0045_38a0, |_, words| {
                assert_eq!(words, &[0x011d_96cc, 0]);
                Ret::default()
            });
            e.register_double(0x0045_38c0, |_, words| {
                assert_eq!(words, &[0x011d_96cc]);
                Ret::default()
            });
            e.register_double(0x0082_5c00, |_, words| {
                assert_eq!(words, &[0x011f_6394]);
                Ret {
                    eax: 42,
                    ..Ret::default()
                }
            });
            e.register_double(0x0040_1000, |e, words| {
                assert_eq!(words, &[0x324]);
                Ret {
                    eax: e.mem.alloc(0x324),
                    ..Ret::default()
                }
            });
            e.register_double(0x005a_8080, move |_, words| {
                assert_eq!(words, &[0x1234]);
                Ret {
                    eax: flags_ptr,
                    ..Ret::default()
                }
            });
            e.register_double(0x0059_e400, move |_, words| {
                assert_eq!(words, &[0x1234]);
                Ret {
                    eax: u32::from(completed),
                    ..Ret::default()
                }
            });
            e.register_double(0x005c_bb50, move |_, words| {
                assert_eq!(words, &[0x7777]);
                Ret {
                    eax: active,
                    ..Ret::default()
                }
            });
            e.register_double(0x0095_29d0, |_, _| Ret::default());
            e.register_double(0x0076_bad0, move |e, words| {
                assert_eq!(e.mem.u32(words[0]), 0x1234);
                assert_eq!(e.mem.u32(words[0] + 4), expected_state);
                Ret::default()
            });
            e.call_log = Some(vec![]);
            hud_main_menu_show_quest_name(&mut e, Ptr::new(0x1234));
            assert_eq!(e.mem.u32(0x011d_96ac), 42);
            let updates: Vec<_> = e
                .call_log
                .as_ref()
                .unwrap()
                .iter()
                .filter(|(address, _)| *address == 0x0095_29d0)
                .collect();
            assert_eq!(updates.len(), usize::from(expected_active.is_some()));
            if let Some(target) = expected_active {
                assert_eq!(&updates[0].1, &[0x7777, target]);
            }
        }
    }

    #[test]
    fn quest_flag_helper_reads_only_the_0x40_bit() {
        let mut e = Engine::new();
        let flags = e.mem.alloc(1);
        e.register_double(0x005a_8080, move |_, words| {
            assert_eq!(words, &[0x1234]);
            Ret {
                eax: flags,
                ..Ret::default()
            }
        });
        e.mem.set_u8(flags, 0x80);
        assert!(!fn_0077a590(&mut e, Ptr::new(0x1234)));
        e.mem.set_u8(flags, 0xc0);
        assert!(fn_0077a590(&mut e, Ptr::new(0x1234)));
    }

    #[test]
    fn objective_queue_copies_text_and_two_status_bytes() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        hud_main_menu_show_quest_objective(&mut e, Ptr::new(0x1234), 1, 2);
        let hud = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, hud.addr());
        e.register_double(0x0045_38a0, |_, words| {
            assert_eq!(words, &[0x011d_9730, 0]);
            Ret::default()
        });
        e.register_double(0x0045_38c0, |_, words| {
            assert_eq!(words, &[0x011d_9730]);
            Ret::default()
        });
        e.register_double(0x0082_5c00, |_, words| {
            assert_eq!(words, &[0x011f_6394]);
            Ret {
                eax: 77,
                ..Ret::default()
            }
        });
        e.register_double(0x0040_1000, |e, words| {
            assert_eq!(words, &[0x106]);
            Ret {
                eax: e.mem.alloc(0x106),
                ..Ret::default()
            }
        });
        e.register_double(0x0040_6d30, |e, words| {
            assert_eq!(&words[1..], &[0x104, 0x1234]);
            e.mem.set_u8(words[0], b'Q');
            Ret::default()
        });
        e.register_double(0x005a_e3d0, move |e, words| {
            assert_eq!(words[0], hud.addr() + 0x258);
            let entry = e.mem.u32(words[1]);
            assert_eq!(e.mem.u8(entry), b'Q');
            assert_eq!(e.mem.u8(entry + 0x104), 9);
            assert_eq!(e.mem.u8(entry + 0x105), 4);
            Ret::default()
        });
        hud_main_menu_show_quest_objective(&mut e, Ptr::new(0x1234), 9, 4);
        assert_eq!(e.mem.u32(0x011d_96ac), 77);
    }

    #[test]
    fn kill_quest_objectives_clears_active_tile_traits() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        e.map(0x5000, 0x1000);
        e.map(0x6000, 4);
        let hud = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, hud.addr());
        e.mem.set_u32(hud.addr() + 0x5c, 0x5000);
        e.mem.set_i32(hud.addr() + 0x1c4, 0);
        e.mem.set_u32(0x6000, 0x7000);
        e.register_double(0x0072_6070, move |_, words| {
            if words[0] == 0x5004 {
                Ret {
                    eax: 1,
                    ..Ret::default()
                }
            } else {
                assert_eq!(words, &[hud.addr() + 0x258]);
                Ret::default()
            }
        });
        e.register_double(0x0068_3520, |e, words| {
            assert_eq!(words[0], 0x5004);
            e.mem.set_u32(words[1], 0);
            Ret {
                eax: 0x6000,
                ..Ret::default()
            }
        });
        e.register_double(0x0070_0320, |_, words| {
            assert_eq!(words, &[0x7000, 0xfa3, 0]);
            Ret::default()
        });
        e.register_double(0x00a0_12d0, |_, words| {
            assert_eq!(words[0], 0x7000);
            assert_eq!(words[2..], [0.0_f32.to_bits(), 1]);
            Ret::default()
        });
        e.register_double(0x00a0_1860, |_, words| {
            let trait_id = match words[0] {
                0x0107_3724 => 0x123,
                0x0107_3718 => 0x124,
                address => panic!("unexpected text trait {address:#x}"),
            };
            Ret {
                eax: trait_id,
                ..Ret::default()
            }
        });
        e.register_double(0x0082_56d0, move |_, words| {
            assert_eq!(words, &[hud.addr() + 0x258]);
            Ret::default()
        });
        e.register_double(0x0068_15c0, move |_, words| {
            assert_eq!(words, &[hud.addr() + 0x258]);
            Ret {
                eax: 0x6000,
                ..Ret::default()
            }
        });
        e.register_double(0x0040_1030, |_, words| {
            assert_eq!(words, &[0x7000]);
            Ret::default()
        });
        e.register_double(0x0047_0470, move |_, words| {
            assert_eq!(words, &[hud.addr() + 0x258]);
            Ret::default()
        });

        hud_main_menu_kill_quest_objectives(&mut e, hud);
        assert_eq!(e.mem.u32(0x011d_96c0), hud.addr());
    }

    #[test]
    fn shake_uses_cached_origin_and_resets_the_one_frame_amount() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        e.map(0x0101_2000, 0x1000);
        e.mem.set_f64(0x0101_2060, 0.0);
        let hud = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(hud.addr() + 4, 0x5000);
        e.mem.set_f32(hud.addr() + 0x274, 0.5);
        e.mem.set_u32(0x011d_9e9c, 3);
        e.mem.set_u8(0x011d_9e91, 0);
        e.mem.set_f32(0x011d_9e98, 100.0);
        e.mem.set_f32(0x011d_9e94, 200.0);
        let writes = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let captured = writes.clone();
        e.register_double(0x004a_4240, |_, words| {
            assert!(words.is_empty());
            Ret {
                st0: 6.0,
                ..Ret::default()
            }
        });
        e.register_double(0x0041_69a0, |e, words| {
            assert_eq!(words[0], 6.0_f32.to_bits());
            assert_eq!(words.len(), 3);
            assert_ne!(words[1], words[2]);
            e.mem.set_f32(words[1], 2.0);
            e.mem.set_f32(words[2], -4.0);
            e.mem.set_f64(0x0101_2060, 0.5);
            Ret::default()
        });
        e.register_double(0x00a0_12d0, move |_, words| {
            captured.borrow_mut().push(words.to_vec());
            Ret::default()
        });

        fn_0077f700(&mut e, hud);
        assert_eq!(
            &*writes.borrow(),
            &[
                vec![0x5000, 0xfa1, 101.0_f32.to_bits(), 1],
                vec![0x5000, 0xfa2, 198.0_f32.to_bits(), 1]
            ]
        );
        assert_eq!(e.mem.f32(hud.addr() + 0x274), 0.0);
        assert_eq!(e.mem.u8(0x011d_9e91), 0);

        writes.borrow_mut().clear();
        fn_0077f700(&mut e, hud);
        assert!(writes.borrow().is_empty());
        assert_eq!(e.mem.u8(0x011d_9e91), 0);
    }

    #[test]
    fn shake_reloads_lazy_flags_root_tile_and_final_amount_after_callbacks() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        e.map(0x0101_2000, 0x1000);
        e.mem.set_f64(0x0101_2060, 0.0);
        let hud = e.new_object::<HUDMainMenu>();
        let first_root = 0x5100;
        let second_root = 0x5200;
        e.mem.set_u32(hud.addr() + 4, first_root);
        e.mem.set_u32(0x011d_9e9c, 0);
        e.mem.set_u8(0x011d_9e91, 1);
        e.mem.set_f32(hud.addr() + 0x274, 0.0);
        let traits = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let observed_traits = traits.clone();
        e.register_double(0x00a0_11b0, move |e, words| {
            observed_traits.borrow_mut().push(words.to_vec());
            match words {
                [tile, 0xfa1] if *tile == first_root => {
                    e.mem.set_u32(0x011d_9e9c, 0);
                    e.mem.set_u32(hud.addr() + 4, second_root);
                    Ret {
                        st0: 10.0,
                        ..Ret::default()
                    }
                }
                [tile, 0xfa2] if *tile == second_root => Ret {
                    st0: 20.0,
                    ..Ret::default()
                },
                args => panic!("unexpected shake trait getter args: {args:?}"),
            }
        });
        let writes = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let observed_writes = writes.clone();
        e.register_double(0x00a0_12d0, move |e, words| {
            let mut captured = observed_writes.borrow_mut();
            captured.push(words.to_vec());
            if captured.len() == 1 {
                e.mem.set_f32(hud.addr() + 0x274, 1.0);
            }
            Ret::default()
        });

        fn_0077f700(&mut e, hud);

        assert_eq!(
            &*traits.borrow(),
            &[vec![first_root, 0xfa1], vec![second_root, 0xfa2]]
        );
        assert_eq!(
            &*writes.borrow(),
            &[
                vec![second_root, 0xfa1, 10.0_f32.to_bits(), 1],
                vec![second_root, 0xfa2, 20.0_f32.to_bits(), 1],
            ]
        );
        assert_eq!(e.mem.u8(0x011d_9e91), 1);
        assert_eq!(e.mem.f32(hud.addr() + 0x274), 0.0);
        assert_eq!(e.mem.u32(0x011d_9e9c), 2);
    }

    #[test]
    fn shift_ammo_tiles_reads_current_hud_and_sets_each_target() {
        let mut e = Engine::new();
        e.map(0x011d_9000, 0x1000);
        let hud = e.new_object::<HUDMainMenu>();
        e.mem.set_u32(0x011d_96c0, hud.addr());
        let offsets = [0x50_u32, 0x54, 0x4c, 0x180];
        for (index, offset) in offsets.into_iter().enumerate() {
            e.mem.set_u32(hud.addr() + offset, 0x5000 + index as u32);
        }
        e.register_double(0x00a0_11b0, |_, words| {
            assert_eq!(words[1], 0xfa1);
            Ret {
                st0: (words[0] - 0x5000 + 1) as f64 * 10.0,
                ..Ret::default()
            }
        });
        let writes = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let captured = writes.clone();
        e.register_double(0x00a0_12d0, move |_, words| {
            captured.borrow_mut().push(words.to_vec());
            Ret::default()
        });

        fn_0077f890(&mut e, hud, -50);
        assert_eq!(
            &*writes.borrow(),
            &[
                vec![0x5000, 0xfa1, (-40.0_f32).to_bits(), 1],
                vec![0x5001, 0xfa1, (-30.0_f32).to_bits(), 1],
                vec![0x5002, 0xfa1, (-20.0_f32).to_bits(), 1],
                vec![0x5003, 0xfa1, (-10.0_f32).to_bits(), 1],
            ]
        );
    }

    #[test]
    fn helper_constructor_sets_vtable_and_forwards_zero_arguments() {
        let mut e = Engine::new();
        let object = e.mem.alloc(4);
        e.register_double(0x006b_3eb0, move |_, words| {
            assert_eq!(words, &[object, 0, 0]);
            Ret::default()
        });
        let result = fn_0077f970(&mut e, Ptr::new(object));
        assert_eq!(result.addr(), object);
        assert_eq!(e.mem.u32(object), 0x0107_38cc);
    }

    #[test]
    fn helper_mode_destructor_sets_vtable_and_forwards_mode_one() {
        let mut e = Engine::new();
        let object = e.mem.alloc(4);
        e.register_double(0x0084_54f0, move |_, words| {
            assert_eq!(words, &[object, 1]);
            Ret::default()
        });
        fn_0077f9a0(&mut e, Ptr::new(object));
        assert_eq!(e.mem.u32(object), 0x0107_38cc);
    }

    #[test]
    fn helper_scalar_destructor_clears_before_conditional_free() {
        let mut e = Engine::new();
        let object = e.mem.alloc(4);
        let calls = std::rc::Rc::new(std::cell::RefCell::new(Vec::new()));
        let clear_calls = calls.clone();
        e.register_double(0x0084_54f0, move |_, words| {
            clear_calls.borrow_mut().push((0x0084_54f0, words.to_vec()));
            Ret::default()
        });
        let free_calls = calls.clone();
        e.register_double(0x0040_1030, move |_, words| {
            free_calls.borrow_mut().push((0x0040_1030, words.to_vec()));
            Ret::default()
        });

        assert_eq!(fn_0077f9d0(&mut e, Ptr::new(object), 2).addr(), object);
        assert_eq!(&*calls.borrow(), &[(0x0084_54f0, vec![object, 1])]);
        calls.borrow_mut().clear();

        assert_eq!(fn_0077f9d0(&mut e, Ptr::new(object), 3).addr(), object);
        assert_eq!(
            &*calls.borrow(),
            &[(0x0084_54f0, vec![object, 1]), (0x0040_1030, vec![object]),]
        );
    }
}
