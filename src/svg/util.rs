//! Small shared helpers for the card builders: XML escaping, greedy word-wrap,
//! and stacked text lines.

pub fn esc(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// Greedy word-wrap to at most `max` chars per line.
pub fn wrap(text: &str, max: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let mut cur = String::new();
    for word in text.split_whitespace() {
        if cur.is_empty() {
            cur = word.to_string();
        } else if cur.chars().count() + 1 + word.chars().count() <= max {
            cur.push(' ');
            cur.push_str(word);
        } else {
            lines.push(std::mem::take(&mut cur));
            cur = word.to_string();
        }
    }
    if !cur.is_empty() {
        lines.push(cur);
    }
    lines
}

/// Render wrapped lines as stacked `<text>` elements.
pub fn lines(text: &str, x: f32, y0: f32, dy: f32, size: f32, fill: &str, max: usize) -> String {
    wrap(text, max)
        .into_iter()
        .enumerate()
        .map(|(i, ln)| {
            let y = y0 + i as f32 * dy;
            format!("<text x=\"{x}\" y=\"{y}\" font-size=\"{size}\" fill=\"{fill}\">{}</text>", esc(&ln))
        })
        .collect()
}
