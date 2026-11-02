# HTML text in the game's menus

A text tile whose `ishtml` trait is above 0 isn't laid out like other
text: the game runs it through a small HTML layout of its own. Read from
FalloutNV.exe 1.4.0.525; the code is `ui::html`.

## Who uses it

- The text tile's update (`00a21af0`, `TileText`'s vtable `01094878` + 8)
  takes the HTML path when `ishtml` is above 0 (or a font manager flag,
  `011f33f8` + 0x20, which nothing sets): `00a18f00` → `00a18a30` → the
  parser `00a17390`. Everything else goes through the plain layout
  (`00a12fb0`, `00a12880`; `ui::text`).
- In code only the tutorial menu sets `ishtml` (`007e9060`): on its text
  tile, when the message's `DESC` starts with `<`. In the menu files only
  `book_menu.xml`'s three page texts have `<ishtml>`; the book menu isn't
  reached in New Vegas (`world::items::read_book`). Message boxes, notes,
  terminals and loading screens are plain text.
- Vanilla's only text written in HTML is `HelpHealingLimbs` (and its Xbox
  copy `HelpHealingLimbsXBox`): `ShowTutorialMenu`'s message in the
  `CGTutorial` quest, page 9 of the help manual. Other messages that start
  with `<` (a debug message's `<DEBUG 010>`, a note's `<MEMO>`, terminals'
  `<!>WARNING…`) are never in an `ishtml` tile.

## The parser (`00a17390`)

- `Font::CollectTo` (`00a16ea0`) reads up to a stop character, with the
  classes of `00a16da0`: the end, `<` or `{`, `>` or `}`, whitespace
  (any byte below `!` as a signed char, so bytes from 0x80 too), a quote,
  `=`. Line ends (`\r`, `\n`) are skipped. In text, tag names and values,
  `&name;` becomes what `007070c0` gives for `name` (a game setting, a
  control's key name, `PCName`…; `007073d0` takes off the `&`, a `-` and
  the `;`); a value starting with `\` is an inline button picture (not
  drawn here); none leaves `&name;` as it is; a `&` without its `;` is
  dropped ("Font::CollectTo() -- Found GameSetting w/o ending ; in HTML
  text.").
- Text that doesn't start (after whitespace) with `<` or `{` isn't parsed:
  `00a18a30` lays it out one character at a time in the tile's font,
  justification and colour, curly quotes straight, `\n` a line break,
  no settings put in; no text is one space.
- Tag names, attribute names and values not in quotes are upper-cased. A
  quoted value is read to the closing quote and the character after it is
  taken too. An attribute without `=` is `true`. Whitespace before `>`
  leaves the `>` to be read as text.
- Tags: `BR` one line break, `P` two, `HR` two when the tile has no
  `wraplimit`, else the next character starts a new page; `/FONT` back to
  font 1, the default colour (117, 59, 33) and left alignment. Attributes:
  `DIV` — one line break for each attribute it has, `ALIGN` = `LEFT`,
  `CENTER`, `RIGHT` for the lines that start after it; `FONT` — `FACE` a
  font file (`sFontFile_N`, as configured) or number 1–8, `COLOR` six hex
  digits; `IMG` — `SRC`, `WIDTH`, `HEIGHT` (`sscanf "%i"`), placed with the
  next text (or the next other tag's attribute). Closing tags other than
  `/FONT`, and every other tag, do nothing.
- The text starts in font 1, not the tile's font, left aligned.

## The layout (`00a19a10`, `00a19c00`, `00a19f70`)

- Each character is an element: its advance is trunc(width + left +
  right kerning), 0 for a glyph without width; its height trunc(the
  font's line height); below its line trunc(−the font's lowest
  "baseline − height"). A picture is `WIDTH` wide, `HEIGHT` high.
- Lines start at a font's height from the table at `011a709c` (35, 35,
  30, 60, 60, 0, 0, 0 for fonts 1–8) and grow to a character's height
  where the font changes and to a picture's.
- Line breaks: n breaks start a new line, n − 1 times "the last
  character's line height + its height − baseline" (35 at a page's start)
  lower.
- Past the wrap width: a space becomes character 0 (no width) at the next
  line's start; with a space on the line, the characters after the last
  one move down and the space goes (the line's width then keeps the width
  of the last character moved: the game's arithmetic); with none,
  characters move down until a hyphen (font 1) fits after the rest.
- Pages: `wraplimit` (if 2 or more) is a page's height; a line that
  doesn't fit starts a new page. `pagenum` is the page shown and
  `pagecount` is set to the number of pages.
- Drawing (`00a19060`): each line sits its height and gap below the last;
  right-aligned lines end at the wrap width, centred ones are centred in
  it (for the one-at-a-time fallback: around 0, as plain text); each glyph
  is placed as plain text's are (`00a142d0`); a picture stands on its
  line. The tile's `width` and `height` are the widest line and the lines'
  heights and gaps on the page shown.
- `COLOR` goes into the glyphs' vertex colours, which the tile shaders
  don't read for colour (`TILE1000.pso` multiplies the texture by
  `TintColor` only; `TILE1002.pso` takes only the vertex alpha): on screen
  HTML text has the tile's colour.

## Here

`ui::html` (`parse`, `plain`, `draw`), used by `Ui::layout` for any text
tile with `ishtml` (so the tutorial menu's `HelpHealingLimbs`, and any
menu file's `<ishtml>` tile); `TextLayout` keeps each quad's font and the
pictures; `draw_list` makes one text item per font and draws the
pictures. Seen live: `ShowTutorialMenu HelpHealingLimbs` (shots
`tut-script.png` before, `html-healinglimbs-after.png` after).

Not done: the inline button pictures (`\…` values, `00a1aee0`; only the
Xbox copy's `&sXBXBtn;` would use them); `FACE` matches the default font
files, not ones set in an INI.
