//! notsu — the profile is *compiled*, not assembled.
//!
//! Fetches live GitHub data and renders two self-contained animated SVGs
//! (`hero.svg`, `pulse.svg`). Static identity lives here; live stats come from
//! `github.rs`. `--preview` uses a committed fixture so motion can be tuned
//! locally without hitting the API. On any live failure it degrades to the
//! fixture rather than emitting a broken card.

mod github;
mod model;
mod svg;
mod theme;

use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;

use model::Profile;

/// The embedded fixture — also the last-good fallback when a live fetch fails.
const FIXTURE: &str = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/fixtures/sample.json"));

#[derive(Parser)]
#[command(name = "notsu", version, about = "Renders the notsu profile SVGs")]
struct Args {
    /// Render from the committed fixture instead of live GitHub data.
    #[arg(long)]
    preview: bool,

    /// GitHub login to fetch.
    #[arg(long, default_value = "notsu")]
    login: String,

    /// Output directory for hero.svg / pulse.svg.
    #[arg(long, default_value = "generated")]
    out: PathBuf,
}

/// Static identity — the parts GitHub can't tell us. Placeholder copy lives in
/// the fixture / README, never fabricated here.
fn base_profile() -> Profile {
    Profile {
        name: "Pichet Itngam".into(),
        handle: "notsu".into(),
        role: "Head of Engineering".into(),
        company: "Fastwork".into(),
        location: "Bangkok, Thailand".into(),
        years: 16,
        commits_year: 0,
        current_streak: 0,
        longest_streak: 0,
        followers: 0,
        stars: 0,
        weeks: Vec::new(),
        langs: Vec::new(),
    }
}

fn fixture() -> Result<Profile> {
    serde_json::from_str(FIXTURE).context("fixture JSON is malformed")
}

fn resolve(args: &Args) -> Profile {
    if args.preview {
        return fixture().expect("embedded fixture must parse");
    }

    match github::token() {
        Some(tok) => match github::fetch(&args.login, &tok) {
            Ok(stats) if !stats.weeks.is_empty() => {
                let mut p = base_profile();
                p.commits_year = stats.commits_year;
                p.followers = stats.followers;
                p.stars = stats.stars;
                p.current_streak = stats.current_streak;
                p.longest_streak = stats.longest_streak;
                p.weeks = stats.weeks;
                p.langs = stats.langs;
                p
            }
            Ok(_) => {
                eprintln!("warning: live fetch returned empty data — using fixture");
                fixture().expect("embedded fixture must parse")
            }
            Err(e) => {
                eprintln!("warning: live fetch failed ({e}) — using fixture");
                fixture().expect("embedded fixture must parse")
            }
        },
        None => {
            eprintln!("warning: no GH_STATS_PAT/GITHUB_TOKEN — using fixture");
            fixture().expect("embedded fixture must parse")
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let profile = resolve(&args);

    std::fs::create_dir_all(&args.out)
        .with_context(|| format!("cannot create {}", args.out.display()))?;

    let hero = svg::hero::render(&profile);
    let pulse = svg::pulse::render(&profile);

    std::fs::write(args.out.join("hero.svg"), &hero).context("writing hero.svg")?;
    std::fs::write(args.out.join("pulse.svg"), &pulse).context("writing pulse.svg")?;

    eprintln!(
        "rendered {} commits · {} streak · {} langs · {} active cells → {}",
        profile.commits_year,
        profile.current_streak,
        profile.langs.len(),
        profile.active_cells(),
        args.out.display()
    );
    Ok(())
}
