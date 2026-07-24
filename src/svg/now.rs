//! The "Now" card — auto-updates daily from live GitHub activity, but never
//! names a repo or project. A typing headline teases, a "sneak peek" hints at
//! the tech (not the product), and an animated sparkline shows the last 7 days
//! of momentum. CSS-only animation.

use crate::model::Profile;
use crate::svg::icons;
use crate::svg::util::esc;
use crate::theme::*;

const W: u32 = 780;
const H: u32 = 182;

fn sparkline(days: &[u64]) -> String {
    if days.is_empty() {
        return String::new();
    }
    let max = days.iter().copied().max().unwrap_or(1).max(1) as f32;
    let bw = 18.0;
    let gap = 8.0;
    let x0 = W as f32 - 22.0 - (days.len() as f32 * bw + (days.len() as f32 - 1.0) * gap);
    let baseline = 150.0;
    let mut bars = String::new();
    for (i, &v) in days.iter().enumerate() {
        let h = 8.0 + (v as f32 / max) * 54.0;
        let x = x0 + i as f32 * (bw + gap);
        let peak = v as f32 >= max; // tallest day gets full accent
        let (fill, op) = if peak { (ACCENT, "1") } else { (ACCENT, "0.4") };
        bars.push_str(&format!(
            "<rect x=\"{x:.1}\" y=\"{y:.1}\" width=\"{bw}\" height=\"{h:.1}\" rx=\"3\" fill=\"{fill}\" \
opacity=\"{op}\" class=\"bar\" style=\"animation-delay:{d}ms\"/>",
            y = baseline - h,
            d = 300 + i * 80,
        ));
    }
    format!(
        "<text x=\"{lx:.1}\" y=\"78\" font-size=\"10.5\" letter-spacing=\"0.6\" fill=\"{DIM}\">COMMITS · LAST 7 DAYS</text>{bars}",
        lx = x0,
    )
}

pub fn render(p: &Profile) -> String {
    let l1 = p.langs.first().map(|l| l.name.as_str()).unwrap_or("code");
    let peek = match p.langs.get(1).map(|l| l.name.as_str()) {
        Some(l2) => format!("{l1} · {l2} · a dash of AI"),
        None => format!("{l1} · a dash of AI"),
    };
    let stats = format!("{} commits this week · {}-day streak", p.commits_week, p.current_streak);
    let headline = "I'm heads-down on something new.";
    let steps = headline.chars().count();

    let style = format!(
        "{base}\
.type{{clip-path:inset(0 100% 0 0);animation:type 1.4s steps({steps}) .3s both;}}\
.dot{{animation:pulse 1.6s ease-in-out infinite;transform-box:fill-box;transform-origin:center;}}\
.ring{{animation:ring 1.6s ease-out infinite;transform-box:fill-box;transform-origin:center;}}\
.bar{{transform:scaleY(0);transform-box:fill-box;transform-origin:bottom;animation:grow .5s cubic-bezier(.2,.8,.2,1) both;}}\
@keyframes type{{to{{clip-path:inset(0 0 0 0);}}}}\
@keyframes pulse{{0%,100%{{opacity:1;}}50%{{opacity:.4;}}}}\
@keyframes ring{{0%{{opacity:.6;transform:scale(.6);}}100%{{opacity:0;transform:scale(2.4);}}}}\
@keyframes grow{{to{{transform:scaleY(1);}}}}",
        base = base_style()
    );

    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{W}\" height=\"{H}\" \
viewBox=\"0 0 {W} {H}\" role=\"img\" aria-label=\"Now: {aria}. {stats}.\">\
<defs><style>{style}</style></defs>\
<rect x=\"1\" y=\"1\" width=\"{cw}\" height=\"{ch}\" rx=\"14\" fill=\"{CARD_TOP}\" stroke=\"{LINE}\"/>\
{icon}\
<text x=\"52\" y=\"38\" font-size=\"14\" font-weight=\"700\" fill=\"{TEXT}\">NOW</text>\
<text x=\"90\" y=\"38\" font-size=\"12\" fill=\"{DIM}\">// what I'm building</text>\
<circle cx=\"{dx}\" cy=\"33\" r=\"12\" fill=\"{ACCENT}\" opacity=\"0.25\" class=\"ring\"/>\
<circle cx=\"{dx}\" cy=\"33\" r=\"4\" fill=\"{ACCENT}\" class=\"dot\"/>\
<text x=\"{lx}\" y=\"37\" font-size=\"11\" font-weight=\"700\" fill=\"{ACCENT}\" text-anchor=\"end\">LIVE</text>\
<text x=\"26\" y=\"82\" font-size=\"20\" font-weight=\"800\" fill=\"{WHITE}\" class=\"type\">{headline}</text>\
<text x=\"26\" y=\"112\" font-size=\"13.5\" fill=\"{CYAN}\">▸ <tspan fill=\"{DIM}\">these days:</tspan> <tspan fill=\"{TEXT}\">{peek}</tspan></text>\
<text x=\"26\" y=\"136\" font-size=\"13\" fill=\"{DIM}\">Can't say what yet — but it's keeping me up at night (the good kind).</text>\
<text x=\"26\" y=\"162\" font-size=\"12.5\" fill=\"{ACCENT}\">◆ <tspan fill=\"{TEXT}\" font-weight=\"700\">{stats}</tspan></text>\
{spark}\
</svg>",
        cw = W - 2,
        ch = H - 2,
        icon = icons::place("broadcast", 20.0, 22.0, 22.0, ACCENT),
        dx = W - 88,
        lx = W - 22,
        aria = esc(headline),
        stats = esc(&stats),
        headline = esc(headline),
        peek = esc(&peek),
        spark = sparkline(&p.week_days),
    )
}
