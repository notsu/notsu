//! Rust-forged palette + the embedded webfont. Colours live here so the two
//! SVG builders stay visually identical, and the font is inlined as base64 so
//! the profile renders the same on every visitor's machine (GitHub serves the
//! SVG through a proxy with no access to system fonts).

pub const CARD_TOP: &str = "#11151F";
pub const CARD_BOT: &str = "#0E121B";
pub const LINE: &str = "#1E2430";
pub const ACCENT: &str = "#E4572E";
pub const OK: &str = "#3FB950";
pub const TEXT: &str = "#E6EDF3";
pub const WHITE: &str = "#FFFFFF";
pub const DIM: &str = "#8B95A1"; // audit fix: was #6B7280, bumped for contrast
pub const CYAN: &str = "#56B6C2";

/// Contribution heatmap colour ramp, level 0 → 4.
pub const HEAT: [&str; 5] = ["#161B24", "#3A2418", "#7A3A1E", "#C04A1F", "#E4572E"];

const FONT_REGULAR_B64: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/fonts/jbm-regular.b64"
));
const FONT_BOLD_B64: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/assets/fonts/jbm-bold.b64"
));

/// `<style>` block shared by every SVG: the embedded font faces plus the
/// reduced-motion escape hatch. Because we animate with CSS (not SMIL),
/// `prefers-reduced-motion` can freeze everything to its final frame.
pub fn base_style() -> String {
    let reg = strip_ws(FONT_REGULAR_B64);
    let bold = strip_ws(FONT_BOLD_B64);
    format!(
        "@font-face{{font-family:'JBM';font-weight:400;font-style:normal;\
src:url(data:font/woff2;base64,{reg}) format('woff2');}}\
@font-face{{font-family:'JBM';font-weight:700;font-style:normal;\
src:url(data:font/woff2;base64,{bold}) format('woff2');}}\
text,tspan{{font-family:'JBM','SFMono-Regular',Menlo,monospace;}}\
@media (prefers-reduced-motion: reduce){{\
*{{animation:none!important;opacity:1!important;}}\
.type{{clip-path:none!important;}}\
}}"
    )
}

fn strip_ws(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}
