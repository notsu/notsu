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
    pub current_streak: u32,
    pub longest_streak: u32,
    pub followers: u64,
    pub stars: u64,

    /// Contribution levels, 0–4, `weeks[w][day]`.
    pub weeks: Vec<Vec<u8>>,
    pub langs: Vec<Lang>,
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
