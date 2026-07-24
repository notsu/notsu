//! The "live pulse" card: self-rendered contribution heatmap (animated fill
//! sweep) + top-languages bars (animated grow). CSS-only animation.

use crate::model::Profile;
use crate::theme::*;

const W: u32 = 780;
const H: u32 = 190;

// Heatmap card.
const HM_X: f32 = 1.0;
const HM_W: f32 = 468.0;
const CELL: f32 = 6.4;
const GAP: f32 = 1.64;
const GRID_X: f32 = 21.0;
const GRID_Y: f32 = 64.0;

// Languages card.
const LG_X: f32 = 482.0;
const LG_W: f32 = 297.0;
const BAR_W: f32 = 120.0;

fn esc(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

fn heatmap(p: &Profile) -> String {
    let mut cells = String::new();
    for (col, week) in p.weeks.iter().enumerate() {
        for (row, &level) in week.iter().enumerate() {
            let x = GRID_X + col as f32 * (CELL + GAP);
            let y = GRID_Y + row as f32 * (CELL + GAP);
            let color = HEAT[(level.min(4)) as usize];
            let delay = col as f32 * 14.0 + row as f32 * 4.0;
            cells.push_str(&format!(
                "<rect x=\"{x:.1}\" y=\"{y:.1}\" width=\"{CELL}\" height=\"{CELL}\" rx=\"1.4\" \
fill=\"{color}\" class=\"cell\" style=\"animation-delay:{delay:.0}ms\"/>"
            ));
        }
    }

    // less → more legend, filling the space below the grid.
    let ly = 152.0;
    cells.push_str(&format!(
        "<text x=\"{GRID_X}\" y=\"{}\" font-size=\"10.5\" fill=\"{DIM}\">less</text>",
        ly + 8.0
    ));
    for (i, color) in HEAT.iter().enumerate() {
        let x = GRID_X + 34.0 + i as f32 * 12.0;
        cells.push_str(&format!(
            "<rect x=\"{x:.1}\" y=\"{ly:.1}\" width=\"9\" height=\"9\" rx=\"2\" fill=\"{color}\"/>"
        ));
    }
    cells.push_str(&format!(
        "<text x=\"{}\" y=\"{}\" font-size=\"10.5\" fill=\"{DIM}\">more</text>",
        GRID_X + 34.0 + 5.0 * 12.0 + 6.0,
        ly + 8.0
    ));
    cells
}

fn langs(p: &Profile) -> String {
    let mut rows = String::new();
    let lx = LG_X + 20.0;
    // Scale bar width relative to the top language so the leader fills the
    // track and the rest stay proportional — true % stays in the label.
    let max_pct = p.langs.iter().map(|l| l.pct).max().unwrap_or(1).max(1) as f32;
    for (i, lang) in p.langs.iter().take(5).enumerate() {
        let y = 62.0 + i as f32 * 23.0;
        let track_x = lx + 84.0;
        let fill_w = (lang.pct as f32 / max_pct) * BAR_W;
        let pct_x = track_x + BAR_W + 12.0;
        let delay = 0.25 + i as f32 * 0.12;
        rows.push_str(&format!(
            "<text x=\"{lx}\" y=\"{ty:.1}\" font-size=\"12.5\" fill=\"{TEXT}\">{name}</text>\
<rect x=\"{track_x:.1}\" y=\"{by:.1}\" width=\"{BAR_W}\" height=\"7\" rx=\"3.5\" fill=\"#0C1017\"/>\
<rect x=\"{track_x:.1}\" y=\"{by:.1}\" width=\"{fill_w:.1}\" height=\"7\" rx=\"3.5\" fill=\"{color}\" \
class=\"bar\" style=\"animation-delay:{delay:.2}s\"/>\
<text x=\"{pct_x:.1}\" y=\"{ty:.1}\" font-size=\"12\" fill=\"{DIM}\">{pct}%</text>",
            ty = y,
            by = y - 8.5,
            name = esc(&lang.name),
            color = esc(&lang.color),
            pct = lang.pct,
        ));
    }
    rows
}

fn card(x: f32, w: f32, title: &str, meta: &str, body: &str) -> String {
    format!(
        "<rect x=\"{x}\" y=\"1\" width=\"{w}\" height=\"{ch}\" rx=\"14\" fill=\"{CARD_TOP}\" stroke=\"{LINE}\"/>\
<text x=\"{tx}\" y=\"32\" font-size=\"11.5\" letter-spacing=\"0.8\" fill=\"{DIM}\" font-weight=\"700\">{title}</text>\
<text x=\"{mx}\" y=\"32\" font-size=\"11.5\" letter-spacing=\"0.8\" fill=\"{ACCENT}\" font-weight=\"700\" text-anchor=\"end\">{meta}</text>\
{body}",
        ch = H - 2,
        tx = x + 20.0,
        mx = x + w - 20.0,
        title = esc(title),
        meta = esc(meta),
    )
}

pub fn render(p: &Profile) -> String {
    let style = format!(
        "{base}\
.cell{{opacity:0;transform:scale(.4);transform-box:fill-box;transform-origin:center;\
animation:cell .4s ease forwards;}}\
.bar{{transform:scaleX(0);transform-box:fill-box;transform-origin:left center;\
animation:grow 1s cubic-bezier(.2,.8,.2,1) forwards;}}\
@keyframes cell{{to{{opacity:1;transform:scale(1);}}}}\
@keyframes grow{{to{{transform:scaleX(1);}}}}",
        base = base_style()
    );

    let heat_card = card(HM_X, HM_W, "CONTRIBUTION", "52 WEEKS", &heatmap(p));
    let lang_card = card(LG_X, LG_W, "TOP LANGUAGES", "BY BYTES", &langs(p));

    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{W}\" height=\"{H}\" \
viewBox=\"0 0 {W} {H}\" role=\"img\" aria-label=\"Contribution heatmap and top languages for notsu.\">\
<defs><style>{style}</style></defs>\
{heat_card}{lang_card}\
</svg>"
    )
}
