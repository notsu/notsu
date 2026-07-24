//! The data the profile is compiled from. Kept deliberately small and
//! serde-friendly so a fixture file and a live GitHub fetch are interchangeable.

use serde::{Deserialize, Serialize};

/// One programming language slice for the "top languages" bars.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lang {
    pub name: String,
    /// 0–100, already normalised across the languages we keep.
    pub pct: u8,
    /// Hex colour (`#RRGGBB`) used for the bar gradient start.
    pub color: String,
}

/// Everything the SVGs render. `weeks` is a 53×7 grid of contribution
/// *levels* (0–4), oldest week first, matching GitHub's calendar layout.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
    pub name: String,
    pub handle: String,
    pub role: String,
    pub company: String,
    pub location: String,
    /// Years shipping. Rendered in the hero. Placeholder until confirmed.
    pub years: u32,

    pub commits_year: u64,
    /// Contributions in the last 7 days — powers the teasing, live "Now" card.
    pub commits_week: u64,
    pub current_streak: u32,
    pub longest_streak: u32,
    pub followers: u64,
    pub stars: u64,

    /// Contribution levels, 0–4, `weeks[w][day]`.
    pub weeks: Vec<Vec<u8>>,
    /// Raw contribution counts for the last 7 days (chronological) — sparkline.
    pub week_days: Vec<u64>,
    pub langs: Vec<Lang>,
}

/// A side project card (dup, cavemode, cattype, devpon…). Static content.
#[derive(Debug, Clone)]
pub struct Project {
    pub name: String,
    /// Right-aligned label, e.g. "getdup.app" or "building".
    pub label: String,
    /// Icon key from `svg::icons` (e.g. "copy", "spark").
    pub icon: String,
    pub desc: String,
    pub stack: Vec<String>,
}

/// A company card (Fastwork, WorkMotion, LINE, Dek-D). Static content.
#[derive(Debug, Clone)]
pub struct Company {
    pub name: String,
    /// Tenure marker: "now", "consulting", "prev".
    pub when: String,
    pub role: String,
}

impl Profile {
    /// Total contribution cells that are non-empty — a cheap sanity signal
    /// used to decide whether a live fetch produced anything usable.
    pub fn active_cells(&self) -> usize {
        self.weeks
            .iter()
            .flatten()
            .filter(|&&level| level > 0)
            .count()
    }
}
