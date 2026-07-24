//! Side-projects card grid (2×2), branded under devpon. Each card carries the
//! project's real logo over a soft accent glow, an accent underline, and a
//! monospace stack line — crafted, not a generic bordered card. Staggered
//! rise-in, CSS only. The README wraps it in a link + per-project link row.

use crate::model::Project;
use crate::svg::icons;
use crate::svg::logos;
use crate::svg::util::{esc, lines};
use crate::theme::*;

const W: u32 = 780;
const GAP: f32 = 14.0;
const CARD_W: f32 = 382.0;
const CARD_H: f32 = 152.0;
const HEAD: f32 = 48.0;

fn card(x: f32, y: f32, p: &Project, delay: f32, idx: usize) -> String {
    let ix = x + 18.0;
    let iy = y + 20.0;
    let nx = ix + 52.0 + 16.0;
    let clip = format!("pc{idx}");

    let mark = match logos::logo(&p.name) {
        Some(b64) => format!(
            "<clipPath id=\"{clip}\"><rect x=\"{ix}\" y=\"{iy}\" width=\"52\" height=\"52\" rx=\"13\"/></clipPath>\
<circle cx=\"{gx}\" cy=\"{gy}\" r=\"35\" fill=\"url(#glow)\"/>\
<image x=\"{ix}\" y=\"{iy}\" width=\"52\" height=\"52\" clip-path=\"url(#{clip})\" \
preserveAspectRatio=\"xMidYMid slice\" href=\"data:image/png;base64,{b64}\"/>\
<rect x=\"{ix}\" y=\"{iy}\" width=\"52\" height=\"52\" rx=\"13\" fill=\"none\" stroke=\"{LINE}\"/>",
            gx = ix + 26.0,
            gy = iy + 26.0,
        ),
        None => icons::place(&p.icon, ix + 4.0, iy + 4.0, 44.0, ACCENT),
    };

    let tagline = lines(&p.desc, ix, y + 96.0, 16.0, 12.5, DIM, 48);
    let stack = esc(&p.stack.join("  ·  "));

    format!(
        "<g class=\"card\" style=\"animation-delay:{delay:.2}s\">\
<rect x=\"{x}\" y=\"{y}\" width=\"{CARD_W}\" height=\"{CARD_H}\" rx=\"14\" fill=\"url(#card)\" stroke=\"{LINE}\"/>\
{mark}\
<text x=\"{nx}\" y=\"{ny:.0}\" font-size=\"16\" font-weight=\"800\" fill=\"{TEXT}\">{name}</text>\
<rect x=\"{nx}\" y=\"{uy:.0}\" width=\"22\" height=\"2.5\" rx=\"1.25\" fill=\"{ACCENT}\"/>\
<text x=\"{lx:.0}\" y=\"{ly:.0}\" font-size=\"11\" font-weight=\"700\" fill=\"{ACCENT}\" text-anchor=\"end\">↗ {label}</text>\
{tagline}\
<circle cx=\"{sx}\" cy=\"{sdy:.0}\" r=\"2.6\" fill=\"{ACCENT}\"/>\
<text x=\"{stx}\" y=\"{sty:.0}\" font-size=\"11.5\" fill=\"{DIM}\" letter-spacing=\"0.3\">{stack}</text>\
</g>",
        ny = iy + 22.0,
        uy = iy + 30.0,
        lx = x + CARD_W - 16.0,
        ly = iy + 20.0,
        sx = ix + 3.0,
        sdy = y + CARD_H - 21.0,
        stx = ix + 12.0,
        sty = y + CARD_H - 17.0,
        name = esc(&p.name),
        label = esc(&p.label),
    )
}

pub fn render(projects: &[Project]) -> String {
    let cards: String = projects
        .iter()
        .take(4)
        .enumerate()
        .map(|(i, p)| {
            let col = (i % 2) as f32;
            let row = (i / 2) as f32;
            let x = 1.0 + col * (CARD_W + GAP);
            let y = HEAD + row * (CARD_H + GAP);
            card(x, y, p, 0.15 + i as f32 * 0.12, i)
        })
        .collect();

    let h = HEAD + 2.0 * CARD_H + GAP + 6.0;
    let style = format!(
        "{base}\
.card{{opacity:0;transform-box:fill-box;animation:rise .6s cubic-bezier(.2,.7,.2,1) both;}}\
@keyframes rise{{from{{opacity:0;transform:translateY(10px);}}to{{opacity:1;transform:translateY(0);}}}}",
        base = base_style()
    );

    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{W}\" height=\"{h:.0}\" \
viewBox=\"0 0 {W} {h:.0}\" role=\"img\" aria-label=\"Side projects by notsu, collected at devpon.com.\">\
<defs>\
<linearGradient id=\"card\" x1=\"0\" y1=\"0\" x2=\"0\" y2=\"1\"><stop offset=\"0\" stop-color=\"{CARD_TOP}\"/><stop offset=\"1\" stop-color=\"{CARD_BOT}\"/></linearGradient>\
<radialGradient id=\"glow\"><stop offset=\"0\" stop-color=\"{ACCENT}\" stop-opacity=\"0.33\"/><stop offset=\"1\" stop-color=\"{ACCENT}\" stop-opacity=\"0\"/></radialGradient>\
<style>{style}</style>\
</defs>\
<text x=\"1\" y=\"30\" font-size=\"13\" font-weight=\"700\" fill=\"{TEXT}\">SIDE PROJECTS</text>\
<text x=\"120\" y=\"30\" font-size=\"12\" fill=\"{DIM}\">// I build these for the joy of it</text>\
<text x=\"{mx}\" y=\"30\" font-size=\"12\" font-weight=\"700\" fill=\"{ACCENT}\" text-anchor=\"end\">devpon.com ↗</text>\
{cards}</svg>",
        mx = W - 1,
    )
}
