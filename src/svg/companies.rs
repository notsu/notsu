//! Companies as a horizontal rail of cards: Fastwork (now), WorkMotion
//! (consulting), LINE, Dek-D (prev). Staggered rise-in, CSS only.

use crate::model::Company;
use crate::svg::util::{esc, lines};
use crate::theme::*;

const W: u32 = 780;
const H: u32 = 168;
const GAP: f32 = 12.0;
const CARD_H: f32 = 112.0;
const TOP: f32 = 48.0;

fn when_color(when: &str) -> &'static str {
    match when {
        "now" => ACCENT,
        "consulting" => CYAN,
        _ => DIM,
    }
}

fn card(x: f32, w: f32, c: &Company, delay: f32) -> String {
    let inner = x + 16.0;
    let role = lines(&c.role, inner, TOP + 52.0, 15.5, 11.5, DIM, 22);
    format!(
        "<g class=\"card\" style=\"animation-delay:{delay:.2}s\">\
<rect x=\"{x:.1}\" y=\"{TOP}\" width=\"{w:.1}\" height=\"{CARD_H}\" rx=\"12\" fill=\"{CARD_TOP}\" stroke=\"{LINE}\"/>\
<text x=\"{inner:.1}\" y=\"{ny:.0}\" font-size=\"14.5\" font-weight=\"700\" fill=\"{TEXT}\">{name}</text>\
<text x=\"{bx:.1}\" y=\"{by:.0}\" font-size=\"10\" font-weight=\"700\" fill=\"{wc}\" text-anchor=\"end\">{when}</text>\
{role}</g>",
        ny = TOP + 28.0,
        bx = x + w - 14.0,
        by = TOP + 27.0,
        wc = when_color(&c.when),
        name = esc(&c.name),
        when = esc(&c.when.to_uppercase()),
    )
}

pub fn render(companies: &[Company]) -> String {
    let n = companies.len().max(1) as f32;
    let card_w = (W as f32 - 2.0 - (n - 1.0) * GAP) / n;
    let cards: String = companies
        .iter()
        .enumerate()
        .map(|(i, c)| {
            let x = 1.0 + i as f32 * (card_w + GAP);
            card(x, card_w, c, 0.15 + i as f32 * 0.1)
        })
        .collect();

    let style = format!(
        "{base}\
.card{{opacity:0;transform-box:fill-box;animation:rise .55s cubic-bezier(.2,.7,.2,1) both;}}\
@keyframes rise{{from{{opacity:0;transform:translateY(8px);}}to{{opacity:1;transform:translateY(0);}}}}",
        base = base_style()
    );

    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{W}\" height=\"{H}\" \
viewBox=\"0 0 {W} {H}\" role=\"img\" aria-label=\"Companies: Fastwork, WorkMotion, LINE, Dek-D.\">\
<defs><style>{style}</style></defs>\
<text x=\"1\" y=\"30\" font-size=\"13\" font-weight=\"700\" fill=\"{TEXT}\">COMPANIES</text>\
<text x=\"108\" y=\"30\" font-size=\"12\" fill=\"{DIM}\">// where I've built &amp; led</text>\
{cards}</svg>"
    )
}
