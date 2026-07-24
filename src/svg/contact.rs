//! Contact chips styled as shell commands — `$ open notsu.io` — echoing the
//! hero's `$ cargo run` so the profile bookends as one terminal session. Each
//! is its own small SVG the README wraps in a link: delightful, on-theme, and
//! nothing like a generic button. A trailing brand glyph aids recognition.

use crate::svg::icons;
use crate::svg::util::esc;
use crate::theme::*;

const H: f32 = 44.0;

/// A single command chip: `$ <command>` with a trailing brand glyph.
pub fn button(command: &str, icon: &str) -> String {
    let cmd_w = command.chars().count() as f32 * 9.0;
    // prompt + command + glyph, with padding.
    let w = 18.0 + 14.0 + cmd_w + 12.0 + 22.0 + 16.0;

    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{w:.0}\" height=\"{H:.0}\" \
viewBox=\"0 0 {w:.0} {H:.0}\" role=\"img\" aria-label=\"{aria}\">\
<defs><style>{base}</style></defs>\
<rect x=\"1\" y=\"1\" width=\"{rw:.1}\" height=\"{rh:.1}\" rx=\"10\" fill=\"{CARD_TOP}\" stroke=\"{LINE}\"/>\
<text x=\"16\" y=\"{ty:.1}\" font-size=\"15\" font-weight=\"800\" fill=\"{ACCENT}\">$</text>\
<text x=\"32\" y=\"{ty:.1}\" font-size=\"14\" fill=\"{TEXT}\">{cmd}</text>\
{glyph}\
</svg>",
        rw = w - 2.0,
        rh = H - 2.0,
        base = base_style(),
        ty = H / 2.0 + 5.0,
        cmd = esc(command),
        glyph = icons::place(icon, w - 30.0, 11.0, 22.0, DIM),
        aria = esc(command),
    )
}
