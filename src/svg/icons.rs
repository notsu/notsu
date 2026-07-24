//! Custom monoline icons drawn on a 24×24 grid, so the profile never depends on
//! emoji (which render differently per platform) or third-party logos. Each icon
//! is stroke-based and inherits the accent palette.

/// Inner path markup for a named icon, drawn in a 0..24 coordinate box.
pub fn paths(name: &str) -> &'static str {
    match name {
        // section: side projects — a lab flask
        "flask" => "<path d=\"M9 3h6M10 3v6l-5.2 9.3A1.8 1.8 0 0 0 6.4 21h11.2a1.8 1.8 0 0 0 1.6-2.7L14 9V3\"/><path d=\"M7.5 15h9\"/>",
        // section: companies — a building
        "building" => "<path d=\"M4 21V4.5A1.5 1.5 0 0 1 5.5 3h6A1.5 1.5 0 0 1 13 4.5V21\"/><path d=\"M13 9h5.5A1.5 1.5 0 0 1 20 10.5V21M3 21h18M7 7h2M7 11h2M7 15h2M16 13h1M16 17h1\"/>",
        // section: live pulse — an activity line
        "pulse" => "<path d=\"M3 12h4l2.5-7 4.5 14 2.5-7H21\"/>",
        // section: now — a live broadcast
        "broadcast" => "<circle cx=\"12\" cy=\"12\" r=\"2.2\"/><path d=\"M8.2 8.2a5.6 5.6 0 0 0 0 7.6M15.8 8.2a5.6 5.6 0 0 1 0 7.6M5.5 5.5a9.4 9.4 0 0 0 0 13M18.5 5.5a9.4 9.4 0 0 1 0 13\"/>",
        // section: contact — a chat bubble
        "chat" => "<path d=\"M4.5 5A1.5 1.5 0 0 1 6 3.5h12A1.5 1.5 0 0 1 19.5 5v8A1.5 1.5 0 0 1 18 14.5H9l-4.5 4v-4H6A1.5 1.5 0 0 1 4.5 13V5\"/>",
        // project glyphs
        "copy" => "<rect x=\"8\" y=\"8\" width=\"11\" height=\"11\" rx=\"2\"/><path d=\"M5 16V6a2 2 0 0 1 2-2h9\"/>",
        "spark" => "<path d=\"M12 3v4M12 17v4M3 12h4M17 12h4M6 6l2.5 2.5M15.5 15.5 18 18M18 6l-2.5 2.5M8.5 15.5 6 18\"/><circle cx=\"12\" cy=\"12\" r=\"2.5\"/>",
        "keyboard" => "<rect x=\"3\" y=\"6\" width=\"18\" height=\"12\" rx=\"2\"/><path d=\"M7 10h.01M11 10h.01M15 10h.01M7 14h10\"/>",
        "layers" => "<path d=\"M12 3l9 4.8-9 4.8-9-4.8 9-4.8ZM3 13l9 4.8 9-4.8\"/>",
        // contact / social
        "globe" => "<circle cx=\"12\" cy=\"12\" r=\"9\"/><path d=\"M3 12h18M12 3c3 3.6 3 14.4 0 18M12 3c-3 3.6-3 14.4 0 18\"/>",
        "linkedin" => "<rect x=\"3\" y=\"3\" width=\"18\" height=\"18\" rx=\"2\"/><path d=\"M7 10v7M7 7.5v.01M11 17v-4.2a2 2 0 0 1 4 0V17M11 17v-7\"/>",
        "x" => "<path d=\"M5 4l14 16M19.5 4 4.5 20\"/>",
        "github" => "<path d=\"M9 19c-4.2 1.4-4.2-2-6-2.4m12 4.4v-3.6c0-1 .3-1.5-.4-2.2 2.9-.3 5.9-1.4 5.9-6.4a5 5 0 0 0-1.4-3.4 4.6 4.6 0 0 0-.1-3.4s-1.1-.3-3.6 1.3a12.3 12.3 0 0 0-6.4 0C6.6 2.5 5.5 2.8 5.5 2.8a4.6 4.6 0 0 0-.1 3.4A5 5 0 0 0 4 9.6c0 5 3 6.1 5.9 6.4-.7.7-.4 1.2-.4 2.2V21\"/>",
        _ => "",
    }
}

/// Place an icon at (x, y), scaled to `size`, stroked in `color`.
pub fn place(name: &str, x: f32, y: f32, size: f32, color: &str) -> String {
    let scale = size / 24.0;
    format!(
        "<g transform=\"translate({x:.1} {y:.1}) scale({scale:.3})\" fill=\"none\" \
stroke=\"{color}\" stroke-width=\"1.9\" stroke-linecap=\"round\" stroke-linejoin=\"round\">{}</g>",
        paths(name)
    )
}
