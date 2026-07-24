//! The animated hero: a `cargo run` boot sequence that compiles the identity.
//! Animation is pure CSS-in-SVG (works through GitHub's image proxy, honours
//! prefers-reduced-motion). No SMIL, no JS.

use crate::model::Profile;
use crate::theme::*;

const W: u32 = 780;
const H: u32 = 250;
const X: f32 = 28.0; // left text inset
const CH: f32 = 8.4; // JBM advance width at 14px

/// Group thousands: 4812 -> "4,812".
fn group(n: u64) -> String {
    let s = n.to_string();
    let bytes = s.as_bytes();
    let mut out = String::new();
    for (i, b) in bytes.iter().enumerate() {
        if i > 0 && (bytes.len() - i).is_multiple_of(3) {
            out.push(',');
        }
        out.push(*b as char);
    }
    out
}

fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// One coloured span inside a line.
fn t(fill: &str, bold: bool, s: &str) -> String {
    let w = if bold { "700" } else { "400" };
    format!(
        "<tspan fill=\"{}\" font-weight=\"{}\">{}</tspan>",
        fill,
        w,
        esc(s)
    )
}

/// A revealed output line at row `y`, appearing after `delay` seconds.
fn line(y: f32, delay: f32, spans: &str) -> String {
    format!(
        "<text x=\"{X}\" y=\"{y}\" font-size=\"14\" class=\"ln\" \
style=\"animation-delay:{delay}s\">{spans}</text>"
    )
}

pub fn render(p: &Profile) -> String {
    let commits = group(p.commits_year);
    let live_plain = format!(
        "live  {} commits · {}d streak · {} followers · {}★",
        commits,
        p.current_streak,
        group(p.followers),
        group(p.stars)
    );
    let caret_x = X + live_plain.chars().count() as f32 * CH + 3.0;

    let cmd = "$ cargo run --bin notsu";
    let cmd_steps = cmd.chars().count();

    // Body lines.
    let l_cmd = format!(
        "<text x=\"{X}\" y=\"78\" font-size=\"14\" class=\"cmd type\" \
style=\"animation:type .9s steps({cmd_steps}) both\">{}{}</text>",
        t(ACCENT, true, "$ "),
        t(TEXT, false, "cargo run --bin notsu")
    );

    let l1 = line(
        104.0,
        1.05,
        &format!(
            "{}{}{}",
            t(DIM, false, "   Compiling "),
            t(WHITE, true, "identity"),
            t(DIM, false, " v4.0.0 (github.com/notsu)")
        ),
    );
    let l2 = line(
        129.0,
        1.40,
        &format!(
            "{}{}{}{}{}{}{}",
            t(ACCENT, true, "   ▸ "),
            t(WHITE, true, &p.name),
            t(DIM, false, " — "),
            t(TEXT, false, &p.role),
            t(DIM, false, " @ "),
            t(ACCENT, true, &p.company),
            t(OK, true, "  ✓")
        ),
    );
    let l3 = line(
        154.0,
        1.75,
        &format!(
            "{}{}{}{}{}{}{}",
            t(ACCENT, true, "   ▸ "),
            t(CYAN, false, "Rust"),
            t(DIM, false, " · "),
            t(CYAN, false, "Go"),
            t(DIM, false, " · distributed systems · DDD · "),
            t(DIM, false, &format!("{}y, self-taught", p.years)),
            t(OK, true, "  ✓")
        ),
    );
    let l4 = line(
        179.0,
        2.10,
        &format!(
            "{}{}{}{}",
            t(ACCENT, true, "   ▸ "),
            t(TEXT, false, &p.location),
            t(DIM, false, " · shipping since 2009"),
            t(OK, true, "  ✓")
        ),
    );
    let l5 = line(
        204.0,
        2.45,
        &format!(
            "{}{}{}",
            t(DIM, false, "    Finished "),
            t(OK, true, "`ship`"),
            t(DIM, false, " profile [optimized]")
        ),
    );

    // Divider + live line.
    let divider = format!(
        "<line x1=\"{X}\" y1=\"219\" x2=\"{}\" y2=\"219\" stroke=\"#222B39\" \
stroke-width=\"1\" stroke-dasharray=\"3 3\" class=\"ln\" style=\"animation-delay:2.7s\"/>",
        W as f32 - X
    );
    let live = format!(
        "<text x=\"{X}\" y=\"237\" font-size=\"13\" class=\"ln\" style=\"animation-delay:2.8s\">{}{}{}{}{}{}{}{}{}</text>",
        t(DIM, false, "live  "),
        t(TEXT, true, &commits),
        t(DIM, false, " commits · "),
        t(TEXT, true, &p.current_streak.to_string()),
        t(DIM, false, "d streak · "),
        t(TEXT, true, &group(p.followers)),
        t(DIM, false, " followers · "),
        t(TEXT, true, &group(p.stars)),
        t(ACCENT, true, "★")
    );
    let caret = format!(
        "<rect x=\"{caret_x:.0}\" y=\"226\" width=\"8\" height=\"15\" fill=\"{ACCENT}\" \
class=\"caret\" style=\"animation-delay:3.0s\"/>"
    );

    let style = format!(
        "{base}\
.ln{{opacity:0;transform-box:fill-box;animation:rise .5s ease both;}}\
.type{{clip-path:inset(0 100% 0 0);}}\
.caret{{opacity:0;animation:blink 1.05s steps(1) infinite;}}\
@keyframes rise{{from{{opacity:0;transform:translateY(5px);}}to{{opacity:1;transform:translateY(0);}}}}\
@keyframes type{{from{{clip-path:inset(0 100% 0 0);}}to{{clip-path:inset(0 0 0 0);}}}}\
@keyframes blink{{0%,50%{{opacity:1;}}51%,100%{{opacity:0;}}}}",
        base = base_style()
    );

    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{W}\" height=\"{H}\" \
viewBox=\"0 0 {W} {H}\" role=\"img\" aria-label=\"{name} — {role} at {company}. {commits} commits, {streak} day streak.\">\
<defs>\
<linearGradient id=\"card\" x1=\"0\" y1=\"0\" x2=\"0\" y2=\"1\">\
<stop offset=\"0\" stop-color=\"{CARD_TOP}\"/><stop offset=\"1\" stop-color=\"{CARD_BOT}\"/>\
</linearGradient>\
<style>{style}</style>\
</defs>\
<rect x=\"1\" y=\"1\" width=\"{cw}\" height=\"{ch}\" rx=\"14\" fill=\"url(#card)\" stroke=\"{LINE}\"/>\
<rect x=\"1\" y=\"1\" width=\"{cw}\" height=\"40\" rx=\"14\" fill=\"#0C1017\"/>\
<rect x=\"1\" y=\"27\" width=\"{cw}\" height=\"14\" fill=\"#0C1017\"/>\
<line x1=\"1\" y1=\"41\" x2=\"{W}\" y2=\"41\" stroke=\"{LINE}\"/>\
<circle cx=\"24\" cy=\"21\" r=\"5.5\" fill=\"#FF5F56\"/>\
<circle cx=\"42\" cy=\"21\" r=\"5.5\" fill=\"#FFBD2E\"/>\
<circle cx=\"60\" cy=\"21\" r=\"5.5\" fill=\"#27C93F\"/>\
<text x=\"80\" y=\"25\" font-size=\"12\" fill=\"#586274\">notsu@github — zsh</text>\
{l_cmd}{l1}{l2}{l3}{l4}{l5}{divider}{live}{caret}\
</svg>",
        name = esc(&p.name),
        role = esc(&p.role),
        company = esc(&p.company),
        streak = p.current_streak,
        cw = W - 2,
        ch = H - 2,
    )
}
