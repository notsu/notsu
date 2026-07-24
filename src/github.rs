//! Live data via the GitHub GraphQL API. Everything is computed here (streaks,
//! language shares, heatmap levels) — no third-party stat services. Uses a PAT
//! (`GH_STATS_PAT`) so private-repo contributions are included; falls back to
//! `GITHUB_TOKEN`.

use anyhow::{anyhow, Context, Result};
use serde_json::json;

use crate::model::Lang;

/// The live fields the generator fills in around the static identity.
pub struct Stats {
    pub commits_year: u64,
    pub commits_week: u64,
    pub followers: u64,
    pub stars: u64,
    pub current_streak: u32,
    pub longest_streak: u32,
    pub weeks: Vec<Vec<u8>>,
    pub week_days: Vec<u64>,
    pub langs: Vec<Lang>,
}

pub fn token() -> Option<String> {
    std::env::var("GH_STATS_PAT")
        .ok()
        .filter(|t| !t.is_empty())
        .or_else(|| std::env::var("GITHUB_TOKEN").ok().filter(|t| !t.is_empty()))
}

const QUERY: &str = r#"
query($login:String!){
  user(login:$login){
    followers{ totalCount }
    contributionsCollection{
      contributionCalendar{
        totalContributions
        weeks{ contributionDays{ contributionCount } }
      }
    }
    repositories(ownerAffiliations:OWNER, isFork:false, first:100, orderBy:{field:PUSHED_AT, direction:DESC}){
      nodes{
        stargazerCount
        pushedAt
        languages(first:12, orderBy:{field:SIZE, direction:DESC}){
          edges{ size node{ name color } }
        }
      }
    }
  }
}"#;

/// Markup / config "languages" that aren't interesting on a dev profile.
const EXCLUDED_LANGS: &[&str] = &[
    "HTML", "CSS", "SCSS", "Less", "Blade", "Makefile", "Dockerfile", "Shell",
    "Batchfile", "PowerShell", "Roff", "Vim Script", "Procfile", "Mustache",
];

pub fn fetch(login: &str, token: &str) -> Result<Stats> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("notsu-profile-generator")
        .build()?;

    let resp = client
        .post("https://api.github.com/graphql")
        .bearer_auth(token)
        .json(&json!({ "query": QUERY, "variables": { "login": login } }))
        .send()
        .context("GraphQL request failed")?;

    let status = resp.status();
    let body: serde_json::Value = resp.json().context("invalid JSON from GitHub")?;
    if !status.is_success() {
        return Err(anyhow!("GitHub returned {status}: {body}"));
    }
    if let Some(errors) = body.get("errors") {
        return Err(anyhow!("GraphQL errors: {errors}"));
    }

    let user = body
        .pointer("/data/user")
        .ok_or_else(|| anyhow!("no user in response"))?;

    let followers = user
        .pointer("/followers/totalCount")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);

    let calendar = user
        .pointer("/contributionsCollection/contributionCalendar")
        .ok_or_else(|| anyhow!("no contribution calendar"))?;

    let commits_year = calendar
        .pointer("/totalContributions")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);

    // Flatten day counts (chronological) and build the 53x7 grid.
    let mut day_counts: Vec<u64> = Vec::new();
    let mut weeks_counts: Vec<Vec<u64>> = Vec::new();
    if let Some(weeks) = calendar.pointer("/weeks").and_then(|v| v.as_array()) {
        for week in weeks {
            let mut wc = Vec::new();
            if let Some(days) = week.pointer("/contributionDays").and_then(|v| v.as_array()) {
                for day in days {
                    let c = day
                        .pointer("/contributionCount")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(0);
                    wc.push(c);
                    day_counts.push(c);
                }
            }
            weeks_counts.push(wc);
        }
    }

    let max = day_counts.iter().copied().max().unwrap_or(0);
    let weeks: Vec<Vec<u8>> = weeks_counts
        .iter()
        .map(|w| w.iter().map(|&c| level(c, max)).collect())
        .collect();

    let (current_streak, longest_streak) = streaks(&day_counts);
    let commits_week: u64 = day_counts.iter().rev().take(7).sum();
    let mut week_days: Vec<u64> = day_counts.iter().rev().take(7).copied().collect();
    week_days.reverse(); // chronological

    // Stars are all-time; languages are computed only from repos active in the
    // last ~6 months, so the chart reflects what Pichet is actually into now.
    let cutoff = cutoff_date();
    let mut stars = 0u64;
    let mut lang_size: std::collections::HashMap<String, (u64, String)> =
        std::collections::HashMap::new();
    if let Some(nodes) = user.pointer("/repositories/nodes").and_then(|v| v.as_array()) {
        for repo in nodes {
            stars += repo
                .pointer("/stargazerCount")
                .and_then(|v| v.as_u64())
                .unwrap_or(0);

            let recent = repo
                .pointer("/pushedAt")
                .and_then(|v| v.as_str())
                .map(|p| p >= cutoff.as_str())
                .unwrap_or(false);
            if !recent {
                continue;
            }

            if let Some(edges) = repo.pointer("/languages/edges").and_then(|v| v.as_array()) {
                for edge in edges {
                    let size = edge.pointer("/size").and_then(|v| v.as_u64()).unwrap_or(0);
                    let name = edge
                        .pointer("/node/name")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    let color = edge
                        .pointer("/node/color")
                        .and_then(|v| v.as_str())
                        .unwrap_or("#E4572E")
                        .to_string();
                    if name.is_empty() || EXCLUDED_LANGS.contains(&name.as_str()) {
                        continue;
                    }
                    let entry = lang_size.entry(name).or_insert((0, color));
                    entry.0 += size;
                }
            }
        }
    }

    let langs = top_langs(lang_size);

    Ok(Stats {
        commits_year,
        commits_week,
        followers,
        stars,
        current_streak,
        longest_streak,
        weeks,
        week_days,
        langs,
    })
}

/// ISO date (`YYYY-MM-DD`) ~6 months ago, for the language recency window.
fn cutoff_date() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let days = (secs / 86_400).saturating_sub(182) as i64; // ~6 months
    let (y, m, d) = civil_from_days(days);
    format!("{y:04}-{m:02}-{d:02}")
}

/// Days-since-Unix-epoch -> (year, month, day). Howard Hinnant's algorithm.
fn civil_from_days(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = (doy - (153 * mp + 2) / 5 + 1) as u32;
    let m = if mp < 10 { mp + 3 } else { mp - 9 } as u32;
    (if m <= 2 { y + 1 } else { y }, m, d)
}

/// Contribution count -> heatmap level 0..4 (quartiles of the year's max).
fn level(count: u64, max: u64) -> u8 {
    if count == 0 || max == 0 {
        return 0;
    }
    let r = count as f64 / max as f64;
    if r <= 0.25 {
        1
    } else if r <= 0.5 {
        2
    } else if r <= 0.75 {
        3
    } else {
        4
    }
}

/// (current, longest) run of days with >0 contributions. Today is allowed a
/// grace: a trailing zero (the current, not-yet-active day) doesn't break the
/// current streak.
fn streaks(days: &[u64]) -> (u32, u32) {
    let mut longest = 0u32;
    let mut run = 0u32;
    for &c in days {
        if c > 0 {
            run += 1;
            longest = longest.max(run);
        } else {
            run = 0;
        }
    }

    let mut current = 0u32;
    let mut iter = days.iter().rev().peekable();
    if let Some(&&last) = iter.peek() {
        if last == 0 {
            iter.next(); // grace for "today"
        }
    }
    for &c in iter {
        if c > 0 {
            current += 1;
        } else {
            break;
        }
    }

    (current, longest)
}

/// Top 5 languages by aggregated size, as percentage of total.
fn top_langs(map: std::collections::HashMap<String, (u64, String)>) -> Vec<Lang> {
    let total: u64 = map.values().map(|(s, _)| *s).sum();
    if total == 0 {
        return Vec::new();
    }
    let mut items: Vec<(String, u64, String)> =
        map.into_iter().map(|(n, (s, c))| (n, s, c)).collect();
    items.sort_by_key(|it| std::cmp::Reverse(it.1));
    items
        .into_iter()
        .take(5)
        .map(|(name, size, color)| Lang {
            name,
            pct: ((size as f64 / total as f64) * 100.0).round() as u8,
            color,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn level_quartiles() {
        assert_eq!(level(0, 10), 0);
        assert_eq!(level(5, 0), 0); // guard against div-by-zero
        assert_eq!(level(2, 10), 1);
        assert_eq!(level(5, 10), 2);
        assert_eq!(level(7, 10), 3);
        assert_eq!(level(10, 10), 4);
    }

    #[test]
    fn streak_counts_trailing_run() {
        // ...1,1,1 at the end => current 3
        let days = [0, 1, 0, 1, 1, 1];
        assert_eq!(streaks(&days), (3, 3));
    }

    #[test]
    fn streak_grants_today_grace() {
        // trailing zero (today, not yet active) must not break the current run
        let days = [1, 1, 1, 0];
        assert_eq!(streaks(&days).0, 3);
    }

    #[test]
    fn longest_is_max_run() {
        let days = [1, 1, 0, 1, 1, 1, 1, 0, 1];
        let (current, longest) = streaks(&days);
        assert_eq!(longest, 4);
        assert_eq!(current, 1);
    }

    #[test]
    fn top_langs_sorted_and_normalised() {
        let mut m = std::collections::HashMap::new();
        m.insert("Rust".to_string(), (300u64, "#DEA584".to_string()));
        m.insert("Go".to_string(), (100u64, "#00ADD8".to_string()));
        let langs = top_langs(m);
        assert_eq!(langs[0].name, "Rust");
        assert_eq!(langs[0].pct, 75);
        assert_eq!(langs[1].pct, 25);
    }
}
