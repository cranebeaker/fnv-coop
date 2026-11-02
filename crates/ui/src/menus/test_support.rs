//! Menus built from scratch for the tests: a font whose glyphs are all the
//! same size, and small menu files laid out like the game's (the tile names,
//! ids and traits its code relies on), so the menus' code can be run
//! without the game's files.

use crate::font::tests::font_bytes;
use crate::font::Font;
use crate::menu::{self, MenuCode};
use crate::tile::{Screen, SystemColors, TileId, Ui};

/// A font: every printable character 10 wide and 20 high (baseline 16),
/// the space 5 wide, line height 24.
pub fn font() -> Font {
    let mut glyphs: Vec<(u8, f32, f32, f32, f32, f32)> = vec![(b' ', 0.0, 0.0, 0.0, 5.0, 0.0)];
    for c in 0x21u8..0x7f {
        glyphs.push((c, 10.0, 20.0, 0.0, 0.0, 16.0));
    }
    Font::parse(&font_bytes(24.0, &glyphs)).unwrap()
}

/// A 1920 × 1080 screen with that font in every slot, and the exe's own
/// text settings (`game::EXE_TEXT_SETTINGS`).
pub fn ui() -> Ui {
    let mut ui = Ui::new(
        Screen {
            width_px: 1920,
            height_px: 1080,
            safe_x: 15.0,
            safe_y: 15.0,
        },
        SystemColors::new(None, None),
        Box::new(|name| crate::game::exe_text_setting(name).map(str::to_string)),
    );
    for slot in ui.fonts.iter_mut() {
        *slot = Some(font());
    }
    ui
}

/// Loads a menu file given as text for a menu's code.
pub fn load(ui: &mut Ui, xml: &str, code: &mut dyn MenuCode) -> TileId {
    let text = xml.to_string();
    let mut read = |p: &str| (p == "test.xml").then(|| text.clone().into_bytes());
    menu::load(ui, &mut read, "test.xml", code, 2.0).unwrap()
}

/// The parts of `list_box.xml` the list code reads, for a list named
/// `name` with `id`.
pub fn list_box(name: &str, id: i32, width: f32, height_ops: &str) -> String {
    format!(
        "<hotrect name=\"{name}\"><id>{id}</id><locus>&true;</locus><clipwindow>&true;</clipwindow>
           <_enabled>&true;</_enabled><_scroll_delta>26</_scroll_delta><_highlight_y>-1</_highlight_y>
           <_selected_height>0</_selected_height><_num_filtered>0</_num_filtered>
           <_number_of_visible_items>0</_number_of_visible_items>
           <width>{width}</width><height>{height_ops}</height>
           <hotrect name=\"lb_highlight_box\"><target>&false;</target>
             <y><copy src=\"parent()\" trait=\"_highlight_y\"/></y>
             <height><copy src=\"parent()\" trait=\"_selected_height\"/></height>
             <image name=\"top\"><filename>solid.dds</filename></image></hotrect>
           <image name=\"lb_scrollbar\"><_number_of_items>1</_number_of_items><_current_value>0</_current_value></image>
         </hotrect>"
    )
}

/// The parts of `list_box_template.xml` an item reads.
pub const LIST_ITEM: &str = "<height>0</height><_VerticalSpacing>20</_VerticalSpacing>
    <_enabled>&true;</_enabled><target>&true;</target><locus>&true;</locus><clips>&true;</clips>
    <xdefault>1</xdefault>
    <y><copy src=\"me()\" trait=\"_y\"/></y>";

/// A message menu laid out like `message_menu.xml`.
pub fn message_menu() -> String {
    format!(
        "<menu name=\"MessageMenu\"><class>&MessageMenu;</class>
           <_MinMenuWidth>200</_MinMenuWidth><_MaxMenuWidth>700</_MaxMenuWidth>
           <_horbuf>40</_horbuf><_verbuf>20</_verbuf>
           <rect name=\"NOGLOW_BRANCH\">
             <image name=\"MM_Background\"><id>5</id><filename>black.dds</filename></image>
             <rect name=\"MM_MainRect\"><id>0</id><locus>&true;</locus>
               <text name=\"MM_Title\"><id>1</id><font>6</font></text>
               <image name=\"MM_MessageIcon\"><id>2</id><visible>&false;</visible></image>
               <text name=\"MM_MessageText\"><id>3</id><font>2</font></text>
               {}
             </rect>
           </rect>
           <template name=\"MM_ButtonTemplate\"><hotrect name=\"MM_Button\">{LIST_ITEM}
             <text name=\"ListItemText\"><font>2</font><string><copy src=\"parent()\" trait=\"string\"/></string></text>
           </hotrect></template>
         </menu>",
        list_box("MM_ButtonList", 4, 0.0, "<min>420</min>")
    )
}

/// A dialogue menu laid out like `dialog_menu.xml`.
pub fn dialog_menu() -> String {
    format!(
        "<menu name=\"DialogMenu\"><class>&DialogMenu;</class>
           <_DialogVisible>&false;</_DialogVisible><_ShowingText>&true;</_ShowingText>
           <_ShowSubtitles>&true;</_ShowSubtitles><_MinListHeight>110</_MinListHeight>
           <_ScrollbarHeight>200</_ScrollbarHeight>
           <rect name=\"NOGLOW_BRANCH\">
             <hotrect name=\"DM_ClickRect\"><id>0</id><width>1706</width><height>960</height>
               <target><copy src=\"io()\" trait=\"_ShowingText\"/></target></hotrect>
             <text name=\"DM_SpeakerNameLabel\"><id>1</id><font>7</font>
               <visible><copy src=\"io()\" trait=\"_DialogVisible\"/></visible></text>
             <text name=\"DM_SpeakerText\"><id>2</id><font>6</font>
               <visible><copy src=\"io()\" trait=\"_ShowingText\"/><and src=\"io()\" trait=\"_DialogVisible\"/></visible></text>
             {}
           </rect>
           <template name=\"DM_TopicTemplate\"><hotrect name=\"DM_Topic\">{LIST_ITEM}<_line_alpha></_line_alpha>
             <text name=\"ListItemText\"><font>6</font><string><copy src=\"parent()\" trait=\"string\"/></string>
               <alpha><copy src=\"parent()\" trait=\"_line_alpha\"/></alpha></text>
           </hotrect></template>
         </menu>",
        list_box(
            "DM_TopicList",
            3,
            1010.0,
            "<max><copy src=\"io()\" trait=\"_MinListHeight\"/></max>"
        )
    )
}

/// A quantity menu laid out like `quantity_menu.xml` (its meter's
/// formulas as the file has them).
pub fn quantity_menu() -> String {
    "<menu name=\"QuantityMenu\"><class>&QuantityMenu;</class>
       <rect name=\"NOGLOW_BRANCH\">
         <image name=\"QM_Background\"><filename>black.dds</filename><width>740</width><height>370</height></image>
         <hotrect name=\"QM_MainRect\"><locus>&true;</locus><x>100</x><y>100</y><target>&true;</target>
           <text name=\"QM_HowManyText\"><font>2</font><string>&-sHowMany;</string></text>
           <rect name=\"QM_AmountMeter\"><id>0</id><x>149.5</x><y>115</y><width>391</width><height>24</height>
             <draggable>&true;</draggable><target>&true;</target>
             <_Value><copy>0</copy>
               <add><copy src=\"me()\" trait=\"user1\"/><onlyif src=\"me()\" trait=\"dragx\"/></add>
               <add><copy src=\"me()\" trait=\"user2\"/><div src=\"me()\" trait=\"user0\"/><onlyifnot src=\"me()\" trait=\"dragx\"/></add>
             </_Value>
             <user0>1</user0>
             <user1><copy src=\"me()\" trait=\"dragx\"/><sub>140</sub><div>392.5</div><max>0</max><min>1</min></user1>
             <user2></user2><user3></user3><user4></user4>
             <image name=\"MeterBackground\"><filename>solid.dds</filename><width>391</width><height>24</height>
               <target><copy src=\"parent()\" trait=\"target\"/></target></image>
           </rect>
           <image name=\"QM_DecreaseArrow\"><id>1</id><filename>a.dds</filename><x>130</x><y>115</y><width>17.5</width><height>35</height><target>&true;</target></image>
           <image name=\"QM_IncreaseArrow\"><id>2</id><filename>a.dds</filename><x>549</x><y>115</y><width>17.5</width><height>35</height><target>&true;</target></image>
           <text name=\"QM_AmountChosen\"><id>3</id><font>6</font>
             <string><copy src=\"sibling(QM_AmountMeter)\" trait=\"_Value\"/><mul src=\"sibling(QM_AmountMeter)\" trait=\"user0\"/><round>1</round></string></text>
           <hotrect name=\"QM_OKButton\"><id>4</id><target>&true;</target><string>&-sOK;</string></hotrect>
           <hotrect name=\"QM_CancelButton\"><id>5</id><target>&true;</target><string>&-sCancel;</string></hotrect>
         </hotrect>
       </rect>
     </menu>"
        .to_string()
}

/// A container menu laid out like `container_menu.xml`: every tile its
/// code needs (ids 0 to 18), two lists, the item template.
pub fn container_menu() -> String {
    let card: String = (13..=18)
        .map(|id| format!("<rect name=\"Card{id}\"><id>{id}</id><visible>&false;</visible></rect>"))
        .collect();
    format!(
        "<menu name=\"ContainerMenu\"><class>&ContainerMenu;</class><_PCButton_A>CM_TakeAllButton</_PCButton_A>
           <rect name=\"NOGLOW_BRANCH\">
             <rect name=\"CM_ItemsRect\"><locus>&true;</locus><width>464</width><height>480</height>
               <image name=\"CM_Items_LeftFilterArrow\"><id>0</id><filename>a.dds</filename><width>17.5</width><height>35</height><target>&true;</target><visible>&true;</visible></image>
               <text name=\"CM_ItemsTitle\"><id>1</id><font>6</font><target>&true;</target><wheelable>&true;</wheelable></text>
               <image name=\"CM_Items_RightFilterArrow\"><id>2</id><filename>a.dds</filename><x>60</x><width>17.5</width><height>35</height><target>&true;</target><visible>&true;</visible></image>
               <text name=\"CM_Items_CapsLabel\"><id>3</id><font>2</font></text>
               {}
             </rect>
             <rect name=\"CM_ContainerRect\"><locus>&true;</locus><x>560</x><width>464</width><height>480</height>
               <image name=\"CM_Container_LeftFilterArrow\"><id>5</id><filename>a.dds</filename><width>17.5</width><height>35</height><target>&true;</target><visible>&true;</visible></image>
               <text name=\"CM_ContainerTitle\"><id>6</id><font>6</font><target>&true;</target><wheelable>&true;</wheelable></text>
               <image name=\"CM_Container_RightFilterArrow\"><id>7</id><filename>a.dds</filename><x>60</x><width>17.5</width><height>35</height><target>&true;</target><visible>&true;</visible></image>
               {}
             </rect>
             <image name=\"CM_ArrowIcon\"><id>9</id><visible>&false;</visible><user0></user0></image>
             <image name=\"CM_TakeAllButton\"><id>10</id><target>&true;</target><filename>a.dds</filename></image>
             <image name=\"CM_ExitButton\"><id>11</id><target>&true;</target><filename>a.dds</filename></image>
             <image name=\"CM_ItemIcon\"><id>12</id><visible>&false;</visible></image>
             {card}
             <text name=\"CM_Subtitle\"><string>x</string></text>
           </rect>
           <template name=\"CM_list_template\"><hotrect name=\"CM_list_template_container\">{LIST_ITEM}<id>20</id>
             <text name=\"ListItemText\"><font>2</font><wrapwidth>370</wrapwidth><string></string><alpha>255</alpha></text>
             <image name=\"CM_list_template_ItemMarker\"><filename>square_filled.dds</filename><visible>&false;</visible></image>
           </hotrect></template>
         </menu>",
        list_box("CM_Items_InventoryList", 4, 462.0, "365"),
        list_box("CM_Container_InventoryList", 8, 464.0, "365"),
    )
}

/// A level-up menu laid out like `levelup_menu.xml`: ids 0 to 9, the two
/// lists shown and enabled by the page, Continue usable when the page's
/// points are given and saying "Done" on the last page, Back past page 0,
/// and the two templates with the file's arrow and marker rules.
pub fn levelup_menu() -> String {
    let list = |name: &str, id: i32, page: i32| {
        list_box(name, id, 470.0, "676").replace(
            "<_enabled>&true;</_enabled>",
            &format!(
                "<visible><copy src=\"io()\" trait=\"_CurrentPage\"/><eq>{page}</eq></visible>
                 <_enabled><copy src=\"me()\" trait=\"visible\"/></_enabled>"
            ),
        )
    };
    let total =
        "<copy src=\"parent()\" trait=\"_BaseValue\"/><add src=\"parent()\" trait=\"_ExtraValue\"/>
                 <add src=\"parent()\" trait=\"_AddedValue\"/>";
    format!(
        "<menu name=\"LevelUpMenu\"><class>&LevelUpMenu;</class><stackingtype>&no_click_past;</stackingtype>
           <_CurrentPage>0</_CurrentPage><_EndPage>1</_EndPage><_CurrPoints></_CurrPoints><_MaxPoints></_MaxPoints>
           <_PCButton_R>LUM_ResetButton</_PCButton_R><_PCButton_A>LUM_ContinueButton</_PCButton_A>
           <_PCButton_E>LUM_BackButton</_PCButton_E>
           <rect name=\"NOGLOW_BRANCH\">
             <rect name=\"LUM_MainRect\"><locus>&true;</locus><width>1024</width><height>800</height>
               <text name=\"LUM_Headline_Title\"><id>0</id><font>6</font></text>
               {}
               {}
               <image name=\"LUM_SelectionIcon\"><id>3</id><visible>&false;</visible></image>
               <image name=\"stats_icon_badge\"><id>9</id><visible>&false;</visible><filename></filename></image>
               <text name=\"LUM_SelectionText\"><id>4</id><font>2</font></text>
               <text name=\"LUM_PointCounter\"><id>5</id><font>6</font></text>
               <hotrect name=\"LUM_ResetButton\"><id>6</id><target>&true;</target><string>&-sReset;</string></hotrect>
               <hotrect name=\"LUM_ContinueButton\"><id>7</id>
                 <target><copy src=\"io()\" trait=\"_CurrPoints\"/><eq src=\"io()\" trait=\"_MaxPoints\"/></target>
                 <string><copy src=\"me()\" trait=\"_Title\"/></string>
                 <_Title><copy src=\"io()\" trait=\"_CurrentPage\"/><eq src=\"io()\" trait=\"_EndPage\"/>
                   <copy src=\"me()\" trait=\"_Title_\"/></_Title>
                 <_Title_0>&-sContinue;</_Title_0><_Title_1>&-sDone;</_Title_1>
                 <clicksound>UIMenuOK</clicksound></hotrect>
               <hotrect name=\"LUM_BackButton\"><id>8</id><target>&true;</target>
                 <visible><copy src=\"io()\" trait=\"_CurrentPage\"/><gt>0</gt></visible>
                 <string>&-sBack;</string></hotrect>
             </rect>
           </rect>
           <template name=\"LUM_SkillTemplate\"><hotrect name=\"LUM_SkillItem\">{LIST_ITEM}
             <_MinValue>0</_MinValue><_MaxValue>100</_MaxValue><_BaseValue></_BaseValue><_ExtraValue></_ExtraValue>
             <_AddedValue></_AddedValue><_OverflowValue></_OverflowValue><_DisplayString></_DisplayString>
             <text name=\"ListItemText\"><font>2</font><string><copy src=\"parent()\" trait=\"string\"/></string></text>
             <text name=\"LUM_Template_ItemValue\"><font>2</font><x>340</x>
               <string><copy src=\"parent()\" trait=\"_DisplayString\"/></string></text>
             <image name=\"LUM_Template_LeftArrow\"><id>13</id><x>245</x><width>64</width><height>64</height>
               <target>&true;</target><clicksound>UIMenuPrevNext</clicksound>
               <visible>{total}<gt src=\"parent()\" trait=\"_MinValue\"/>
                 <and><copy src=\"parent()\" trait=\"_AddedValue\"/><gt>0</gt></and></visible></image>
             <image name=\"LUM_Template_RightArrow\"><id>14</id><x>370</x><width>64</width><height>64</height>
               <target>&true;</target><clicksound>UIMenuPrevNext</clicksound>
               <visible>{total}<lt src=\"parent()\" trait=\"_MaxValue\"/>
                 <and><copy src=\"io()\" trait=\"_CurrPoints\"/><lt src=\"io()\" trait=\"_MaxPoints\"/></and></visible></image>
           </hotrect></template>
           <template name=\"LUM_PerkTemplate\"><hotrect name=\"LUM_Perk\">{LIST_ITEM}<_TextAlpha>255</_TextAlpha>
             <clicksound>UIMenuPrevNext</clicksound>
             <text name=\"ListItemText\"><font>2</font><string><copy src=\"parent()\" trait=\"string\"/></string>
               <alpha><copy src=\"parent()\" trait=\"_TextAlpha\"/></alpha></text>
             <image name=\"LUM_Template_ItemMarker\"><filename>square_filled.dds</filename>
               <visible><copy src=\"parent()\" trait=\"_selected\"/></visible></image>
           </hotrect></template>
         </menu>",
        list("LUM_SkillList", 1, 0),
        list("LUM_PerkList", 2, 1),
    )
}

/// A trait menu laid out like `trait_menu.xml`: ids 0 to 8 (the
/// description with its scroll bar), the list, the perk template.
pub fn trait_menu() -> String {
    format!(
        "<menu name=\"TraitMenu\"><class>&TraitMenu;</class><stackingtype>&no_click_past;</stackingtype>
           <_CurrPoints></_CurrPoints><_MaxPoints></_MaxPoints>
           <rect name=\"NOGLOW_BRANCH\">
             <rect name=\"LUM_MainRect\"><locus>&true;</locus><width>1024</width><height>800</height>
               <text name=\"LUM_Headline_Title\"><id>0</id><font>6</font></text>
               {}
               <image name=\"LUM_SelectionIcon\"><id>2</id><visible>&false;</visible></image>
               <image name=\"stats_icon_badge\"><id>7</id><visible>&false;</visible><filename></filename></image>
               <hotrect name=\"TM_DescriptionBox\"><locus>&true;</locus><width>500</width><height>275</height>
                 <hotrect name=\"TM_DescriptionScrollbar\"><id>8</id><_current_value>0</_current_value><_ScrollDelta>20</_ScrollDelta></hotrect>
                 <text name=\"TM_DescriptionText\"><id>3</id><font>2</font><string></string><wrapwidth>480</wrapwidth></text>
               </hotrect>
               <text name=\"LUM_PointCounter\"><id>4</id><font>6</font></text>
               <hotrect name=\"LUM_ResetButton\"><id>5</id><target>&true;</target><string>&-sReset;</string></hotrect>
               <hotrect name=\"LUM_ContinueButton\"><id>6</id><target>&true;</target>
                 <string><copy src=\"me()\" trait=\"_Title\"/></string><_Title>&-sDone;</_Title></hotrect>
             </rect>
           </rect>
           <template name=\"LUM_PerkTemplate\"><hotrect name=\"LUM_Perk\">{LIST_ITEM}<_TextAlpha>255</_TextAlpha>
             <clicksound>UIMenuPrevNext</clicksound>
             <text name=\"ListItemText\"><font>2</font><string><copy src=\"parent()\" trait=\"string\"/></string>
               <alpha><copy src=\"parent()\" trait=\"_TextAlpha\"/></alpha></text>
             <image name=\"LUM_Template_ItemMarker\"><filename>square_filled.dds</filename>
               <visible><copy src=\"parent()\" trait=\"_selected\"/></visible></image>
           </hotrect></template>
         </menu>",
        list_box("LUM_PerkList", 1, 470.0, "676"),
    )
}

/// A character generation menu laid out like `char_gen_menu.xml`: ids 0
/// to 6, Done usable when every point is used, the skills' template.
pub fn char_gen_menu() -> String {
    format!(
        "<menu name=\"CharGenMenu\"><class>&CharGenMenu;</class><stackingtype>&no_click_past;</stackingtype>
           <_CurrPoints></_CurrPoints><_MaxPoints></_MaxPoints><_OnlyAdd>&false;</_OnlyAdd>
           <rect name=\"NOGLOW_BRANCH\">
             <rect name=\"CGM_MainRect\"><locus>&true;</locus><width>1024</width><height>800</height>
               <text name=\"CGM_Headline_Title\"><id>0</id><font>6</font></text>
               {}
               <image name=\"CGM_SelectionIcon\"><id>2</id><visible>&false;</visible></image>
               <text name=\"CGM_SelectionText\"><id>3</id><font>2</font><wrapwidth>540</wrapwidth></text>
               <text name=\"CGM_PointCounter\"><id>4</id><font>2</font></text>
               <hotrect name=\"CGM_ResetButton\"><id>5</id><target>&true;</target><string>&-sReset;</string></hotrect>
               <hotrect name=\"CGM_DoneButton\"><id>6</id>
                 <target><copy src=\"io()\" trait=\"_CurrPoints\"/><eq src=\"io()\" trait=\"_MaxPoints\"/></target>
                 <string>&-sDone;</string></hotrect>
             </rect>
           </rect>
           <template name=\"CGM_SelectItemTemplate\"><hotrect name=\"CGM_SelectItem\">{LIST_ITEM}
             <_MinValue>0</_MinValue><_MaxValue>100</_MaxValue><_BaseValue></_BaseValue><_ExtraValue></_ExtraValue>
             <_AddedValue></_AddedValue><_ValueString></_ValueString><_selected>&false;</_selected>
             <text name=\"ListItemText\"><font>2</font><string><copy src=\"parent()\" trait=\"string\"/></string></text>
             <text name=\"CGM_Template_ItemValue\"><font>2</font><x>360</x>
               <string><copy src=\"parent()\" trait=\"_ValueString\"/></string></text>
             <image name=\"CGM_Template_ItemMarker\"><filename>square_filled.dds</filename>
               <visible><copy src=\"parent()\" trait=\"_selected\"/></visible></image>
           </hotrect></template>
         </menu>",
        list_box("CGM_ItemList", 1, 450.0, "676"),
    )
}

/// A text edit menu laid out like `texteditmenu.xml`: the text (id 0,
/// `wrapwidth` 250), OK (1), the prompt (2).
pub fn text_edit_menu() -> String {
    "<menu name=\"TextEditMenu\"><class>&TextEditMenu;</class><stackingtype>&no_click_past;</stackingtype>
       <rect name=\"TEM_MainRect\"><width>720</width><height>180</height><locus>&true;</locus>
         <text name=\"textedit_prompt\"><id>2</id><string>&-sEnterName;</string></text>
         <text name=\"textedit_text\"><id>0</id><string></string><wrapwidth>250</wrapwidth></text>
         <hotrect name=\"textedit_button_ok\"><id>1</id><string>&-sOk;</string></hotrect>
       </rect>
     </menu>"
        .to_string()
}

/// A barter menu laid out like `barter_menu.xml`: every tile its code
/// keeps (ids 0 to 21), the two lists, and its one template under the
/// file's own name (`BM_list_template`; the code asks for another).
pub fn barter_menu() -> String {
    let card: String = (14..=18)
        .map(|id| format!("<rect name=\"Card{id}\"><id>{id}</id><visible>&false;</visible></rect>"))
        .collect();
    format!(
        "<menu name=\"BarterMenu\"><class>&BarterMenu;</class><_IsContainerListSelected>-1</_IsContainerListSelected>
           <_ArrowDirection></_ArrowDirection><_PCButton_A>BM_ButtonX</_PCButton_A><_PCButton_X>BM_ButtonB</_PCButton_X>
           <rect name=\"NOGLOW_BRANCH\">
             <hotrect name=\"BM_Items_FocusRect\"><id>0</id><width>464</width><height>480</height>
               <target><copy src=\"io()\" trait=\"_IsContainerListSelected\"/></target></hotrect>
             <hotrect name=\"BM_Container_FocusRect\"><id>1</id><x>560</x><width>464</width><height>480</height>
               <target><not src=\"io()\" trait=\"_IsContainerListSelected\"/></target></hotrect>
             <rect name=\"BM_ItemsRect\"><locus>&true;</locus><width>464</width><height>480</height>
               <image name=\"BM_Items_LeftFilterArrow\"><id>2</id><filename>a.dds</filename><target>&true;</target></image>
               <text name=\"BM_ItemsTitle\"><id>3</id><font>6</font><target>&true;</target><wheelable>&true;</wheelable></text>
               <image name=\"BM_Items_RightFilterArrow\"><id>4</id><filename>a.dds</filename><target>&true;</target></image>
               <text name=\"BM_Items_CapsLabel\"><id>5</id><font>2</font></text>
               {}
             </rect>
             <rect name=\"BM_ContainerRect\"><locus>&true;</locus><x>560</x><width>464</width><height>480</height>
               <image name=\"BM_Container_LeftFilterArrow\"><id>7</id><filename>a.dds</filename><target>&true;</target></image>
               <text name=\"BM_ContainerTitle\"><id>8</id><font>6</font><string></string><target>&true;</target></text>
               <image name=\"BM_Container_RightFilterArrow\"><id>9</id><filename>a.dds</filename><target>&true;</target></image>
               <text name=\"BM_Container_CapsLabel\"><id>10</id><font>2</font></text>
               {}
             </rect>
             <rect name=\"BM_CapsFlow\"><id>12</id><visible>&false;</visible><_Value></_Value><alpha>255</alpha>
               <text name=\"BM_CapsLabel\"><string>x</string></text></rect>
             <image name=\"BM_ButtonX\"><id>13</id><target>&false;</target><filename>a.dds</filename></image>
             <image name=\"BM_ButtonB\"><id>19</id><target>&true;</target><filename>a.dds</filename></image>
             <image name=\"BM_ItemIcon\"><id>20</id><visible>&false;</visible></image>
             <rect name=\"BM_ItemData\"><id>21</id>{card}
               <rect name=\"WeightInfo\"></rect><rect name=\"ValueInfo\"></rect></rect>
           </rect>
           <template name=\"BM_list_template\"><hotrect name=\"BM_list_template_container\">{LIST_ITEM}<id>23</id>
             <_Value></_Value><_NumBartered></_NumBartered><_IsEquipped>&false;</_IsEquipped><_IsBarterSelected>&false;</_IsBarterSelected>
             <text name=\"ListItemText\"><font>2</font><string><copy src=\"parent()\" trait=\"string\"/></string><wrapwidth>300</wrapwidth></text>
           </hotrect></template>
         </menu>",
        list_box("BM_Items_InventoryList", 6, 462.0, "365"),
        list_box("BM_Container_InventoryList", 11, 464.0, "365"),
    )
}

/// A sleep/wait menu laid out like `sleep_wait_menu.xml`, with the parts
/// of its `scrollbar_horiz.xml` prefab the bar's value comes from (the
/// arrows, pages, wheel and drag adding onto `_current_value`, kept within
/// 0 and items − visible).
pub fn sleep_wait_menu() -> String {
    "<menu name=\"SleepWaitMenu\"><class>&SleepWaitMenu;</class>
       <stackingtype>&no_click_past;</stackingtype>
       <_PCButton_W>SWM_WaitButton</_PCButton_W><_PCButton_E>SWM_CancelButton</_PCButton_E>
       <rect name=\"NOGLOW_BRANCH\">
         <image name=\"SWM_Background\"><filename>black.dds</filename><width>940</width><height>520</height>
           <alpha><copy src=\"globals()\" trait=\"_background_fill_alpha\"/></alpha></image>
         <hotrect name=\"SWM_MainRect\"><locus>&true;</locus><target>&true;</target><wheelable>&true;</wheelable>
           <x>423</x><y>280</y><width>860</width><height>400</height>
           <text name=\"SWM_HowManyText\"><id>0</id><x>430</x><y>70</y><font>1</font><justify>&center;</justify><string></string></text>
           <hotrect name=\"SWM_Scrollbar\"><id>1</id><xdefault>1</xdefault><locus>&true;</locus><target>&true;</target>
             <_x>112.5</_x><_y>145</_y><_width>640</_width><_height>64</_height>
             <_number_of_items>24</_number_of_items><_number_of_visible_items>1</_number_of_visible_items>
             <_wheelmoved><copy src=\"parent()\" trait=\"wheelmoved\"/></_wheelmoved>
             <_step_size>1</_step_size><_jump_size>6</_jump_size><_enabled>&true;</_enabled><_SetInCode></_SetInCode>
             <_current_value>
               <add><copy src=\"child(scrollbar_horiz_right)\" trait=\"clicked\"/><sub src=\"child(scrollbar_horiz_left)\" trait=\"clicked\"/>
                 <mul src=\"me()\" trait=\"_step_size\"/><onlyif src=\"me()\" trait=\"_enabled\"/></add>
               <add><copy src=\"child(scrollbar_horiz_page_right)\" trait=\"clicked\"/><sub src=\"child(scrollbar_horiz_page_left)\" trait=\"clicked\"/>
                 <mul src=\"me()\" trait=\"_jump_size\"/><onlyif src=\"me()\" trait=\"_enabled\"/></add>
               <add><copy src=\"me()\" trait=\"_wheelmoved\"/><mul src=\"me()\" trait=\"_step_size\"/><mul>-1</mul><onlyif src=\"me()\" trait=\"_enabled\"/></add>
               <onlyif><copy src=\"child(scrollbar_horiz_marker)\" trait=\"dragstartx\"/><lt>0</lt></onlyif>
               <add><copy src=\"child(scrollbar_horiz_marker)\" trait=\"_drag_value\"/>
                 <onlyifnot><copy src=\"child(scrollbar_horiz_marker)\" trait=\"dragstartx\"/><lt>0</lt></onlyifnot></add>
               <min><copy src=\"me()\" trait=\"_number_of_items\"/><sub src=\"me()\" trait=\"_number_of_visible_items\"/></min>
               <max>0</max>
             </_current_value>
             <x><copy src=\"me()\" trait=\"_x\"/></x><y><copy src=\"me()\" trait=\"_y\"/></y>
             <width><copy src=\"me()\" trait=\"_width\"/></width><height><copy src=\"me()\" trait=\"_height\"/></height>
             <image name=\"scrollbar_horiz_page_left\"><alpha>0</alpha><target><copy src=\"parent()\" trait=\"target\"/></target>
               <width><copy src=\"sibling(scrollbar_horiz_marker)\" trait=\"x\"/></width><height>64</height></image>
             <image name=\"scrollbar_horiz_page_right\"><alpha>0</alpha><target><copy src=\"parent()\" trait=\"target\"/></target>
               <x><copy src=\"sibling(scrollbar_horiz_marker)\" trait=\"x\"/><add>48</add></x><width>100</width><height>64</height></image>
             <image name=\"scrollbar_horiz_marker\"><target><copy src=\"parent()\" trait=\"target\"/></target><filename>a.dds</filename>
               <width>48</width><height>48</height><draggable><copy src=\"parent()\" trait=\"_enabled\"/></draggable><dragstartx>-1</dragstartx>
               <x><copy src=\"parent()\" trait=\"_current_value\"/><mul src=\"me()\" trait=\"_step_width\"/><max>0</max></x>
               <_drag_value><copy src=\"me()\" trait=\"dragx\"/><div src=\"me()\" trait=\"_step_width\"/><floor>0</floor></_drag_value>
               <_step_width><copy src=\"parent()\" trait=\"width\"/><sub src=\"me()\" trait=\"width\"/>
                 <div><copy src=\"parent()\" trait=\"_number_of_items\"/><sub src=\"parent()\" trait=\"_number_of_visible_items\"/></div></_step_width>
             </image>
             <image name=\"scrollbar_horiz_left\"><target><copy src=\"parent()\" trait=\"target\"/></target><filename>a.dds</filename><width>0</width><height>0</height></image>
             <image name=\"scrollbar_horiz_right\"><target><copy src=\"parent()\" trait=\"target\"/></target><filename>a.dds</filename><width>0</width><height>0</height><x>640</x></image>
           </hotrect>
           <text name=\"SWM_HoursChosen\"><id>2</id><x>430</x><y>210</y><font>6</font><justify>&center;</justify><string></string></text>
           <text name=\"SWM_CurrentTime\"><id>3</id><x>430</x><y>330</y><font>2</font><justify>&center;</justify><string></string></text>
           <hotrect name=\"SWM_WaitButton\"><id>4</id><target>&true;</target><x>838</x><y>265</y><width>158</width><height>52</height>
             <text name=\"button_text\"><font>2</font><string><copy src=\"parent()\" trait=\"string\"/></string></text></hotrect>
           <hotrect name=\"SWM_CancelButton\"><id>5</id><target>&true;</target><x>838</x><y>310</y><width>99</width><height>52</height>
             <text name=\"button_text\"><font>2</font><string><copy src=\"parent()\" trait=\"string\"/></string></text></hotrect>
         </hotrect>
       </rect>
     </menu>"
        .to_string()
}

/// A recipe menu laid out like `recipe_menu.xml`: the filter (0-2), the
/// two lists (3, 6), the right side's fields (4, 5, 11-13), Accept (7),
/// Exit (8), the picture (9) and card (10), and its only template
/// `RM_list_template` (the code asks for `CM_list_template`).
pub fn recipe_menu() -> String {
    format!(
        "<menu name=\"RecipeMenu\"><class>&RecipeMenu;</class>
           <rect name=\"NOGLOW_BRANCH\">
             <rect name=\"RM_ItemsRect\"><locus>&true;</locus><width>464</width><height>480</height>
               <image name=\"RM_Items_LeftFilterArrow\"><id>0</id><target>&true;</target></image>
               <text name=\"RM_ItemsTitle\"><id>1</id><font>6</font><target>&true;</target></text>
               <image name=\"RM_Items_RightFilterArrow\"><id>2</id><target>&true;</target></image>
               {}
             </rect>
             <rect name=\"RM_ContainerRect\"><locus>&true;</locus><width>464</width><height>480</height>
               <text name=\"RM_ContainerTitle\"><id>12</id><font>6</font></text>
               <text name=\"RM_MadeAtVariable\"><id>4</id><font>3</font></text>
               <text name=\"RM_SkillRequirementHeader\"><id>13</id><font>6</font></text>
               <text name=\"RM_SkillRequirement\"><id>5</id><font>3</font></text>
               <text name=\"RM_Items_IngredientList\"><id>11</id><font>6</font></text>
               {}
             </rect>
             <image name=\"RM_ButtonX\"><id>7</id><x>1273</x><target>&false;</target>
               <text name=\"button_text\"><font>2</font><string><copy src=\"parent()\" trait=\"string\"/></string></text></image>
             <image name=\"RM_ButtonB\"><id>8</id><x>1307</x><target>&true;</target>
               <text name=\"button_text\"><font>2</font><string><copy src=\"parent()\" trait=\"string\"/></string></text></image>
             <image name=\"RM_ItemIcon\"><id>9</id><visible>&false;</visible></image>
             <rect name=\"RM_ItemData\"><id>10</id><visible>&false;</visible><width>530</width><height>120</height>
               <rect name=\"DamageResistInfo\"><_Title></_Title><_Value></_Value><visible>&false;</visible></rect>
               <rect name=\"DPSInfo\"><_Title></_Title><_Value></_Value><visible>&false;</visible></rect>
               <rect name=\"WeightInfo\"><_Title></_Title><_Value></_Value><visible>&false;</visible></rect>
               <rect name=\"ValueInfo\"><_Title></_Title><_Value></_Value><visible>&false;</visible></rect>
               <rect name=\"ConditionInfo\"><_Title></_Title><_Value></_Value><visible>&false;</visible></rect>
               <rect name=\"AmmoInfo\"><_Title></_Title><_Value></_Value><visible>&false;</visible></rect>
               <rect name=\"EffectsInfo\"><_Title></_Title><_Value></_Value><visible>&false;</visible></rect>
               <rect name=\"ModInfoOne\"><_Title></_Title><_Value></_Value><visible>&false;</visible></rect>
               <rect name=\"ModInfoTwo\"><_Title></_Title><_Value></_Value><visible>&false;</visible></rect>
               <rect name=\"ModInfoThree\"><_Title></_Title><_Value></_Value><visible>&false;</visible></rect>
               <rect name=\"StrengthReqInfo\"><_Title></_Title><_Value></_Value><visible>&false;</visible></rect>
               <rect name=\"DAMInfo\"><_Title></_Title><_Value></_Value><visible>&false;</visible></rect>
               <rect name=\"DamageThresholdInfo\"><_Title></_Title><_Value></_Value><visible>&false;</visible></rect>
             </rect>
           </rect>
           <template name=\"RM_list_template\"><hotrect name=\"RM_list_template_container\">{LIST_ITEM}<id>15</id>
             <text name=\"ListItemText\"><font>2</font><string><copy src=\"parent()\" trait=\"string\"/></string></text>
           </hotrect></template>
         </menu>",
        list_box("RM_Items_InventoryList", 3, 462.0, "365"),
        list_box("RM_Items_IngredientList", 6, 442.0, "225"),
    )
}
/// A hacking menu with the tiles, ids, traits and templates the hacking
/// code uses (`hacking_menu.xml`'s structure: the depth rect holding the
/// cursor and the four areas, the lockout's two lines beside it).
pub fn hacking_menu() -> String {
    "<menu name=\"HackingMenu\"><class>&HackingMenu;</class><locus>&true;</locus>
       <user0></user0><user1>0</user1><user2>&false;</user2><user3>0</user3>
       <rect name=\"depth\"><locus>&true;</locus><width>920</width><height>630</height><x>32</x>
         <y><copy>48</copy><sub src=\"parent()\" trait=\"user1\"/></y>
         <hotrect name=\"cursor\"><id>4</id><width>17</width><height>17</height><_blink_interval>400</_blink_interval></hotrect>
         <rect name=\"intro\"><id>0</id><visible>&true;</visible><locus>&true;</locus></rect>
         <rect name=\"header\"><id>1</id><locus>&true;</locus><height>150</height>
           <visible><not src=\"sibling(intro)\" trait=\"visible\"/></visible>
           <user0>&false;</user0><user1></user1><user2></user2></rect>
         <rect name=\"screen\"><id>2</id><locus>&true;</locus><y>150</y><_start_col2>342</_start_col2>
           <visible><not src=\"sibling(intro)\" trait=\"visible\"/></visible>
           <width>665</width><height>476</height>
           <user0></user0><user1>-1</user1><user2></user2><user3></user3><user4>-1</user4><user5></user5></rect>
         <rect name=\"log\"><id>3</id><locus>&true;</locus><x>665</x><y>150</y><width>255</width><height>476</height>
           <visible><not src=\"sibling(intro)\" trait=\"visible\"/></visible>
           <text name=\"prompt\"><font>5</font><x>17</x><y>440</y><string><copy src=\"HackingMenu\" trait=\"user0\"/></string></text>
           <text name=\"entry\"><id>5</id><font>5</font><x>38</x><y>440</y></text></rect>
       </rect>
       <rect name=\"locked\"><locus>&true;</locus><visible><copy src=\"parent()\" trait=\"user2\"/></visible>
         <text name=\"locked1\"><id>6</id><font>5</font></text>
         <text name=\"locked2\"><id>7</id><font>5</font></text></rect>
       <template name=\"hacking_password_file_template\">
         <hotrect name=\"row\"><visible>&false;</visible><target>&true;</target><locus>&true;</locus>
           <width>323</width><height>17</height>
           <x><copy src=\"HackingMenu\" trait=\"user3\"/><add><copy src=\"parent()\" trait=\"_start_col2\"/><mul src=\"me()\" trait=\"user0\"/></add></x>
           <y><copy src=\"me()\" trait=\"height\"/><mul src=\"me()\" trait=\"listindex\"/></y>
           <user0></user0>
           <text name=\"row_text\"><font>5</font><string><copy src=\"parent()\" trait=\"string\"/></string><y>6</y></text></hotrect>
       </template>
       <template name=\"hacking_password_log_template\">
         <text name=\"log_line\"><font>5</font><x>17</x>
           <y><copy>440</copy><sub><copy>20</copy><mul><copy>3</copy><add src=\"me()\" trait=\"listindex\"/></mul></sub></y></text>
       </template>
       <template name=\"hacking_intro_template\">
         <text name=\"intro_text\"><visible>&false;</visible><font>5</font></text>
       </template>
       <template name=\"hacking_guess_template\">
         <hotrect name=\"guess\"><visible><copy src=\"parent()\" trait=\"user0\"/></visible>
           <x><copy src=\"parent()\" trait=\"user1\"/><add><copy src=\"me()\" trait=\"listindex\"/><mul>2</mul><add>1</add><mul src=\"me()\" trait=\"width\"/></add></x>
           <y><copy src=\"parent()\" trait=\"user2\"/></y><width>17</width><height>17</height></hotrect>
       </template>
     </menu>"
        .to_string()
}

/// A terminal menu with the tiles, ids and template the terminal code uses
/// (`computers_menu.xml`'s structure: the logon's four lines, the headers,
/// the welcome and separator, the list, the prompt and result, the cursor,
/// the display zone and its text).
pub fn computers_menu() -> String {
    format!(
        "<menu name=\"ComputersMenu\"><class>&ComputersMenu;</class><locus>&true;</locus>
           <rect name=\"depth\"><locus>&true;</locus><width>960</width><height>720</height>
             <image name=\"background\"><id>13</id><target>&true;</target><width>960</width><height>720</height><filename>a.dds</filename></image>
             <text name=\"logon1\"><id>9</id><x>100</x><y>60</y><font>5</font></text>
             <text name=\"logon2\"><id>10</id><x>100</x><y>108</y><font>5</font></text>
             <text name=\"logon3\"><id>11</id><x>100</x><y>156</y><font>5</font></text>
             <text name=\"logon4\"><id>12</id><x>100</x><y>204</y><font>5</font></text>
             <text name=\"header1\"><id>3</id><y>60</y><font>5</font></text>
             <text name=\"header2\"><id>4</id><y>84</y><font>5</font></text>
             <text name=\"server\"><id>0</id><y>108</y><font>5</font></text>
             <text name=\"welcome\"><id>7</id><visible>&false;</visible><wrapwidth>600</wrapwidth><x>100</x><y>152</y><font>5</font></text>
             <image name=\"separator\"><id>15</id><visible>&false;</visible><x>100</x><y>200</y><filename>a.dds</filename></image>
             {list}
             <text name=\"prompt\"><id>8</id><visible>&false;</visible><font>5</font><x>100</x><y>666</y></text>
             <text name=\"result\"><id>5</id><visible>&false;</visible><font>5</font><x>134</x><y>666</y></text>
             <hotrect name=\"cursor\"><id>6</id><visible>&false;</visible><width>17</width><height>17</height><_blink_interval>400</_blink_interval></hotrect>
             <hotrect name=\"zone\"><id>2</id><locus>&true;</locus><target>&true;</target><visible>&false;</visible><x>100</x><y>220</y><width>700</width><height>400</height>
               <text name=\"zone_text\"><id>14</id><font>5</font><y>10</y><visible>&false;</visible><wrapwidth>600</wrapwidth><wraplines>16</wraplines></text></hotrect>
           </rect>
           <template name=\"computers_file_template\">
             <hotrect name=\"computers_file_template_item\">{item}<user0></user0>
               <text name=\"computers_file_template_text\"><string><copy src=\"parent()\" trait=\"user0\"/></string><font>5</font><y>15</y></text>
             </hotrect>
           </template>
         </menu>",
        list = list_box("computers_file_directory", 1, 700.0, "300"),
        item = LIST_ITEM,
    )
}

/// A merchant's repair menu laid out like `repair_services_menu.xml`: the
/// labels (0, 1), the list (2), the picture (3), the stats (4: condition
/// cards 5 and 10 with `user0`, stat cards 6 and 11, the "+" texts 7 and
/// 8, the error 9 shown while card 10 isn't, the cost 12), the buttons
/// (13 repair, 14 Repair All, 15 Done); a line's text first, its meter
/// (18) and its worn mark last.
/// The companion wheel's tiles as `companion_wheel_menu.xml` has them:
/// eight `radial` slices round the screen's centre (radius 76 to 332;
/// four on the right from 30 to 150 degrees, four on the left from 210 to
/// 330, the file's angles), the texts, Exit.
pub fn companion_wheel_menu() -> String {
    let angles = [
        ("0.523598", "1.047196"),
        ("1.047196", "1.5707943"),
        ("1.5707943", "2.094329"),
        ("2.094329", "2.61799"),
        ("3.665186", "4.188784"),
        ("4.188784", "4.7123823"),
        ("4.7123823", "5.235980"),
        ("5.235980", "5.759578"),
    ];
    let slices: String = angles
        .iter()
        .enumerate()
        .map(|(i, (from, to))| {
            format!(
                "<radial name=\"CWM_Slice{i}\"><id>{i}</id><filename>a.dds</filename>
                   <target>&true;</target><width>760</width><height>760</height>
                   <user0>960</user0><user1>540</user1><user2>{from}</user2><user3>{to}</user3>
                   <user4>76</user4><user5>332</user5><user10>0</user10><user11>0</user11></radial>"
            )
        })
        .collect();
    let text = |name: &str, id: i32| {
        format!("<text name=\"{name}\"><id>{id}</id><font>2</font><string></string></text>")
    };
    format!(
        "<menu name=\"CompanionWheelMenu\"><class>&CompanionWheelMenu;</class>
           <rect name=\"NOGLOW_BRANCH\"><hotrect name=\"CWM_MainRect\"><locus>&true;</locus>
             <x>580</x><y>160</y><width>760</width><height>760</height>
             {}{}{}
             <rect name=\"CWM_ExitButton\"><id>11</id><target>&true;</target><string></string></rect>
             {}{}{}{}{slices}
           </hotrect></rect>
         </menu>",
        text("CWM_RadialLabel", 8),
        text("CWM_ButtonPreviewText", 9),
        text("CWM_ButtonContextText", 10),
        text("CWM_Subtitle", 12),
        text("CWM_ExitCallout", 13),
        text("CWM_SelectCallout", 14),
        text("CWM_NavigateCallout", 15),
    )
}

pub fn repair_services_menu() -> String {
    let card = |name: &str, id: i32, extra: &str| {
        format!(
            "<rect name=\"{name}\"><id>{id}</id><_Title></_Title><_Value></_Value><user0>0</user0>{extra}</rect>"
        )
    };
    format!(
        "<menu name=\"RepairServicesMenu\"><class>&RepairServicesMenu;</class>
           <_PCButton_A>RSM_RepairAllButton</_PCButton_A><_PCButton_E>RSM_DoneButton</_PCButton_E>
           <rect name=\"NOGLOW_BRANCH\"><rect name=\"RSM_MainRect\"><locus>&true;</locus><width>1020</width><height>680</height>
             <text name=\"RSM_VendorSkillLabel\"><id>0</id><string></string><font>2</font></text>
             <text name=\"RSM_CapsLabel\"><id>1</id><string></string><font>2</font></text>
             {}
             <image name=\"RSM_ItemIcon\"><id>3</id><visible>&false;</visible></image>
             <rect name=\"RSM_StatsDisplayRect\"><id>4</id><visible>&false;</visible><locus>&true;</locus>
               {}{}
               <text name=\"RSM_HealthImprovementText\"><id>7</id><font>4</font><string></string>
                 <visible><copy src=\"sibling(RSM_FixedItemHealth)\" trait=\"visible\"/></visible></text>
               <text name=\"RSM_StatImprovementText\"><id>8</id><font>4</font><string></string>
                 <visible><copy src=\"sibling(RSM_FixedItemHealth)\" trait=\"visible\"/></visible></text>
               <text name=\"RSM_ErrorText\"><id>9</id><font>2</font><string></string>
                 <visible><not src=\"sibling(RSM_FixedItemHealth)\" trait=\"visible\"/></visible></text>
               {}{}
               <text name=\"RSM_RepairCostText\"><id>12</id><font>6</font><string></string>
                 <visible><copy src=\"sibling(RSM_FixedItemHealth)\" trait=\"visible\"/></visible></text>
             </rect>
             <image name=\"RSM_RepairButton\"><id>13</id><filename>a.dds</filename><target>&true;</target></image>
             <image name=\"RSM_RepairAllButton\"><id>14</id><filename>a.dds</filename><target>&true;</target><string></string></image>
             <image name=\"RSM_DoneButton\"><id>15</id><filename>a.dds</filename><target>&true;</target></image>
           </rect></rect>
           <template name=\"RSM_RepairListTemplate\"><hotrect name=\"RSM_RepairListTemplateRect\">{LIST_ITEM}
             <_Value></_Value><_RepairCost></_RepairCost><_CanRepair>&true;</_CanRepair>
             <text name=\"ListItemText\"><font>2</font><string><copy src=\"parent()\" trait=\"string\"/></string><wrapwidth>260</wrapwidth></text>
             <image name=\"RSM_Template_MeterBackground\"><filename>solid.dds</filename><width>60</width></image>
             <image name=\"RSM_Template_Meter\"><id>18</id><filename>solid.dds</filename><width>0</width></image>
             <image name=\"RSM_Template_ItemMarker\"><filename>square.dds</filename><visible>&false;</visible></image>
           </hotrect></template>
         </menu>",
        list_box("RSM_RepairList", 2, 480.0, "580"),
        card("RSM_BrokenItemHealth", 5, ""),
        card("RSM_BrokenItemStat", 6, ""),
        card("RSM_FixedItemHealth", 10, "<visible>&false;</visible>"),
        card(
            "RSM_FixedItemStat",
            11,
            "<visible><copy src=\"sibling(RSM_FixedItemHealth)\" trait=\"visible\"/></visible>"
        ),
    )
}

/// A stand-in for `caravan_menu.xml` and its four prefabs: the screens'
/// rects (0–3), the displays (a rect with `_Value`, a title and a value
/// text), the buttons (a hotrect with a text child), the scrollbar, the
/// track value rects (shown later), the info rects the code widens.
pub fn caravan_menu() -> String {
    let display = |name: &str, id: i32| {
        format!(
            "<rect name=\"{name}\"><id>{id}</id><_Value></_Value><string></string>
               <text name=\"{name}Title\"><font>7</font><string></string></text>
               <text name=\"{name}Value\"><font>7</font><string><copy src=\"parent()\" trait=\"_Value\"/></string></text>
             </rect>"
        )
    };
    let button = |name: &str, id: i32| {
        format!(
            "<hotrect name=\"{name}\"><id>{id}</id><target>&true;</target><string></string><_PCButtonText></_PCButtonText>
               <text name=\"{name}Text\"><font>7</font><string><copy src=\"parent()\" trait=\"string\"/></string></text>
             </hotrect>"
        )
    };
    let text = |name: &str, id: i32| {
        format!("<text name=\"{name}\"><id>{id}</id><font>7</font><string></string></text>")
    };
    let tracks: String = (23..29)
        .map(|i| display(&format!("CGM_Track{i}"), i))
        .collect();
    format!(
        "<menu name=\"CaravanMenu\"><class>&CaravanMenu;</class>
           <rect name=\"NOGLOW_BRANCH\">
             <rect name=\"CBM_MainRect\"><id>0</id><width>1000</width>
               <rect name=\"CBM_InfoRect\"><width>300</width>
                 {}{}{}{}{}
                 <text name=\"CBM_NPCBetValue\"><x>20</x><string></string></text>
               </rect>
               {}{}{}{}
             </rect>
             <rect name=\"CDM_MainRect\"><id>1</id><width>1000</width>
               {}{}{}{}{}{}{}{}{}{}
               <hotrect name=\"CDM_Scrollbar\"><id>15</id><user0>0</user0><_current_value>12</_current_value></hotrect>
             </rect>
             <rect name=\"CDG_MainRect\"><id>2</id><width>1600</width>
               <rect name=\"CGM_InfoRect\"><x>1300</x><width>300</width>{}{}{}{}{}{}</rect>
               {}{}{}{}
               <rect name=\"CGM_PlayerTrackRect\"><visible>&false;</visible>{tracks}</rect>
               <rect name=\"CGM_NPCTrackRect\"><visible>&false;</visible></rect>
               {}{}
             </rect>
             <rect name=\"CDR_MainRect\"><id>3</id><width>1000</width>
               <rect name=\"CRM_InfoRect\"><width>300</width>{}{}{}{}{}{}{}{}{}</rect>
             </rect>
           </rect>
         </menu>",
        display("CBM_NPCBetDisplay", 4),
        display("CBM_PlayerBetDisplay", 5),
        display("CBM_TotalFundsDisplay", 6),
        text("CBM_PlayerBetTitle", 40),
        text("CBM_TotalFundsTitle", 41),
        button("CBM_AutoMatchButton", 37),
        button("CBM_RaiseButton", 36),
        button("CBM_AcceptButton", 7),
        button("CBM_ExitButton", 38),
        display("CDM_CardsInDeckDisplay", 8),
        display("CDM_TotalCardsDisplay", 9),
        button("CDM_CaravanDeckButton", 10),
        button("CDM_AllCardsButton", 11),
        button("CDM_AddButton", 12),
        button("CDM_RemoveButton", 13),
        button("CDM_PlayButton", 14),
        button("CDM_NavigateButton", 39),
        text("CDM_CardsInDeckTitle", 42),
        text("CDM_TotalCardsTitle", 43),
        display("CGM_AnteDisplay", 16),
        display("CGM_NetTotalDisplay", 17),
        display("CGM_WinLossDisplay", 18),
        text("CGM_AnteTitle", 44),
        text("CGM_NetTotalTitle", 45),
        text("CGM_WinLossTitle", 46),
        button("CGM_SelectCardButton", 19),
        button("CGM_DiscardCardButton", 20),
        button("CGM_DiscardTrackButton", 21),
        button("CGM_ForfeitGameButton", 22),
        display("CGM_PlayerCardNum", 29),
        display("CGM_NPCCardNum", 30),
        display("CRM_NPCBetDisplay", 31),
        display("CRM_Losses", 32),
        display("CRM_WinLoss", 33),
        display("CRM_LargestWinning", 34),
        text("CRM_Info", 35),
        text("CRM_WinningsTitle", 47),
        text("CRM_LossesTitle", 48),
        text("CRM_WinLossTitle", 49),
        text("CRM_BiggestTitle", 50),
    )
}

/// The slot machine's menu as its file lays it out: the three displays (a
/// rect with `_Value`, its title copying `_Value` and its value `_X`), the
/// six buttons, the status line, the menu's `_PCButton_` traits.
pub fn slots_menu_file() -> String {
    let display = |name: &str, id: i32| {
        format!(
            "<rect name=\"{name}\"><id>{id}</id><_Value></_Value>
               <text name=\"{name}Title\"><font>7</font><string><copy src=\"parent()\" trait=\"_Value\"/></string></text>
               <text name=\"{name}Value\"><font>7</font><string><copy src=\"parent()\" trait=\"_X\"/></string></text>
             </rect>"
        )
    };
    let button = |name: &str, id: i32| {
        format!(
            "<hotrect name=\"{name}\"><id>{id}</id><target>&true;</target><string></string>
               <text name=\"{name}Text\"><font>7</font><string><copy src=\"parent()\" trait=\"string\"/></string></text>
             </hotrect>"
        )
    };
    format!(
        "<menu name=\"SlotMachineMenu\"><class>&SlotMachineMenu;</class>
           <_PCButton_Q>SMM_DecreaseBet</_PCButton_Q><_PCButton_E>SMM_IncreaseBet</_PCButton_E>
           <_PCButton_W>SMM_Spin</_PCButton_W><_PCBUTTON_S>SMM_BetMax</_PCBUTTON_S>
           <_PCBUTTON_R>SMM_ExitButton</_PCBUTTON_R><_PCBUTTON_F>SMM_PayoutList</_PCBUTTON_F>
           <rect name=\"NOGLOW_BRANCH\">
             {}{}
             <rect name=\"SMM_CasinoNameDisplay\"><id>2</id><_Value></_Value>
               <text name=\"SMM_CasinoNameTitle\"><string><copy src=\"parent()\" trait=\"_Value\"/></string></text>
               <text name=\"SMM_TotalEarningsValue\"><string><copy src=\"parent()\" trait=\"_X\"/></string></text>
             </rect>
             {}{}{}{}{}{}
             <text name=\"SMM_StatusText\"><id>9</id><font>7</font><visible>&false;</visible></text>
           </rect>
         </menu>",
        display("SMM_CurrentBetDisplay", 0),
        display("SMM_ChipsDisplay", 1),
        button("SMM_Spin", 3),
        button("SMM_IncreaseBet", 4),
        button("SMM_DecreaseBet", 5),
        button("SMM_PayoutList", 6),
        button("SMM_BetMax", 7),
        button("SMM_ExitButton", 8),
    )
}

/// A stand-in for `black_jack_menu.xml`: the ten tiles by id, each
/// button a hotrect with a text child, the game's key bindings.
pub fn blackjack_menu_file() -> String {
    let display = |name: &str, id: i32| {
        format!(
            "<rect name=\"{name}\"><id>{id}</id><_Value></_Value>
               <text name=\"{name}Title\"><font>7</font><string><copy src=\"parent()\" trait=\"_Value\"/></string></text>
               <text name=\"{name}Value\"><font>7</font><string><copy src=\"parent()\" trait=\"_X\"/></string></text>
             </rect>"
        )
    };
    let button = |name: &str, id: i32| {
        format!(
            "<hotrect name=\"{name}\"><id>{id}</id><target>&true;</target><string></string>
               <text name=\"{name}Text\"><font>7</font><string><copy src=\"parent()\" trait=\"string\"/></string></text>
             </hotrect>"
        )
    };
    format!(
        "<menu name=\"BlackJackMenu\"><class>&BlackJackMenu;</class>
           <_PCButton_F>BJM_Hit</_PCButton_F><_PCButton_Q>BJM_Switch_DecreaseBet</_PCButton_Q>
           <_PCButton_E>BJM_Split_IncreaseBet</_PCButton_E><_PCButton_W>BJM_Double_Deal</_PCButton_W>
           <_PCBUTTON_S>BJM_Surrender_MaxBet</_PCBUTTON_S><_PCBUTTON_R>BJM_Stay_Exit</_PCBUTTON_R>
           <rect name=\"NOGLOW_BRANCH\">
             {}{}
             <rect name=\"BJM_CasinoNameDisplay\"><id>2</id><_Value></_Value>
               <text name=\"BJM_CasinoNameTitle\"><string><copy src=\"parent()\" trait=\"_Value\"/></string></text>
               <text name=\"BJM_TotalEarningsValue\"><string><copy src=\"parent()\" trait=\"_X\"/></string></text>
             </rect>
             {}{}{}{}{}{}
             <text name=\"BJM_StatusText\"><id>9</id><font>7</font><visible>&false;</visible></text>
           </rect>
         </menu>",
        display("BJM_CurrentBetDisplay", 0),
        display("BJM_ChipsDisplay", 1),
        button("BJM_Hit", 3),
        button("BJM_Double_Deal", 4),
        button("BJM_Split_IncreaseBet", 5),
        button("BJM_Switch_DecreaseBet", 6),
        button("BJM_Surrender_MaxBet", 7),
        button("BJM_Stay_Exit", 8),
    )
}

/// A stand-in for `roulette_menu.xml`: the eleven tiles by id, each
/// button a hotrect with a text child, the game's key bindings.
pub fn roulette_menu_file() -> String {
    let display = |name: &str, id: i32| {
        format!(
            "<rect name=\"{name}\"><id>{id}</id><_Value></_Value>
               <text name=\"{name}Title\"><font>7</font><string><copy src=\"parent()\" trait=\"_Value\"/></string></text>
               <text name=\"{name}Value\"><font>7</font><string><copy src=\"parent()\" trait=\"_X\"/></string></text>
             </rect>"
        )
    };
    let button = |name: &str, id: i32| {
        format!(
            "<hotrect name=\"{name}\"><id>{id}</id><target>&true;</target><string></string>
               <text name=\"{name}Text\"><font>7</font><string><copy src=\"parent()\" trait=\"string\"/></string></text>
             </hotrect>"
        )
    };
    format!(
        "<menu name=\"RouletteMenu\"><class>&RouletteMenu;</class>
           <_PCButton_Q>ROM_DecreaseBet</_PCButton_Q><_PCButton_E>ROM_IncreaseBet</_PCButton_E>
           <_PCButton_W>ROM_PlaceBet</_PCButton_W><_PCBUTTON_S>ROM_FinishBet</_PCBUTTON_S>
           <_PCBUTTON_R>ROM_ExitButton</_PCBUTTON_R><_PCBUTTON_F>ROM_RemoveBet</_PCBUTTON_F>
           <rect name=\"NOGLOW_BRANCH\">
             {}{}{}
             <rect name=\"ROM_CasinoNameDisplay\"><id>2</id><_Value></_Value>
               <text name=\"ROM_CasinoNameTitle\"><string><copy src=\"parent()\" trait=\"_Value\"/></string></text>
               <text name=\"ROM_TotalEarningsValue\"><string><copy src=\"parent()\" trait=\"_X\"/></string></text>
             </rect>
             {}{}{}{}{}{}
             <text name=\"ROM_StatusText\"><id>9</id><font>7</font><visible>&false;</visible></text>
           </rect>
         </menu>",
        display("ROM_CurrentBetDisplay", 0),
        display("ROM_TotalBetDisplay", 10),
        display("ROM_ChipsDisplay", 1),
        button("ROM_PlaceBet", 3),
        button("ROM_RemoveBet", 4),
        button("ROM_FinishBet", 5),
        button("ROM_IncreaseBet", 6),
        button("ROM_DecreaseBet", 7),
        button("ROM_ExitButton", 8),
    )
}

/// The roulette menu loaded from [`roulette_menu_file`].
pub fn roulette_menu() -> (Ui, super::roulette::RouletteMenu) {
    let mut ui = self::ui();
    let mut code = super::roulette::RouletteMenu::new(0);
    let menu = load(&mut ui, &roulette_menu_file(), &mut code);
    code.menu = menu;
    (ui, code)
}

/// The blackjack menu loaded from [`blackjack_menu_file`].
pub fn blackjack_menu() -> (Ui, super::blackjack::BlackjackMenu) {
    let mut ui = self::ui();
    let mut code = super::blackjack::BlackjackMenu::new(0);
    let menu = load(&mut ui, &blackjack_menu_file(), &mut code);
    code.menu = menu;
    (ui, code)
}

/// A UI with the slot machine's menu loaded.
pub fn slots_menu() -> (Ui, super::slots::SlotsMenu) {
    let mut ui = self::ui();
    let mut code = super::slots::SlotsMenu::new(0);
    let menu = load(&mut ui, &slots_menu_file(), &mut code);
    code.menu = menu;
    (ui, code)
}
