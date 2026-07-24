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

use model::{Company, Profile, Project};

fn s(v: &str) -> String {
    v.to_string()
}

/// A standalone 26×26 section-heading icon in the accent colour.
fn section_icon(icon: &str) -> String {
    format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"26\" height=\"26\" viewBox=\"0 0 26 26\">{}</svg>",
        svg::icons::place(icon, 2.0, 2.0, 22.0, theme::ACCENT)
    )
}

/// Static side-project cards (not from the API). devpon is the collection site
/// Pichet is founding; the others live under it.
fn projects() -> Vec<Project> {
    vec![
        Project {
            name: s("dup"),
            label: s("getdup.app"),
            icon: s("copy"),
            desc: s("Find & remove duplicate photos and videos on macOS using perceptual hashing."),
            stack: vec![s("Rust"), s("Tauri"), s("Svelte")],
        },
        Project {
            name: s("cavemode"),
            label: s("cavemode.app"),
            icon: s("spark"),
            desc: s("A menu-bar app that tracks Claude + Codex usage — watch your AI devolve."),
            stack: vec![s("Swift"), s("macOS")],
        },
        Project {
            name: s("cattype"),
            label: s("cattype.io"),
            icon: s("keyboard"),
            desc: s("A competitive multiplayer typing game with 9 languages and real-time racing."),
            stack: vec![s("SvelteKit"), s("Cloudflare")],
        },
        Project {
            name: s("devpon"),
            label: s("devpon.com"),
            icon: s("layers"),
            desc: s("The home I'm building to collect all of my side projects in one place."),
            stack: vec![s("founding"), s("2026")],
        },
    ]
}

/// Static company cards.
fn companies() -> Vec<Company> {
    vec![
        Company {
            name: s("Fastwork"),
            when: s("now"),
            role: s("Head of Engineering — leading Thailand's freelance marketplace."),
        },
        Company {
            name: s("WorkMotion"),
            when: s("consulting"),
            role: s("Technical consultant — engineering advice, on & off."),
        },
        Company {
            name: s("LINE"),
            when: s("prev"),
            role: s("Built LINE's SME platform (OAPlus) for small businesses."),
        },
        Company {
            name: s("Dek-D"),
            when: s("prev"),
            role: s("Web & mobile for a Thai student community platform."),
        },
    ]
}

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
        commits_week: 0,
        current_streak: 0,
        longest_streak: 0,
        followers: 0,
        stars: 0,
        weeks: Vec::new(),
        week_days: Vec::new(),
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
                p.commits_week = stats.commits_week;
                p.followers = stats.followers;
                p.stars = stats.stars;
                p.current_streak = stats.current_streak;
                p.longest_streak = stats.longest_streak;
                p.weeks = stats.weeks;
                p.week_days = stats.week_days;
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

    let write = |name: &str, body: &str| -> Result<()> {
        std::fs::write(args.out.join(name), body).with_context(|| format!("writing {name}"))
    };

    write("hero.svg", &svg::hero::render(&profile))?;
    write("pulse.svg", &svg::pulse::render(&profile))?;
    write("projects.svg", &svg::projects::render(&projects()))?;
    write("companies.svg", &svg::companies::render(&companies()))?;
    write("now.svg", &svg::now::render(&profile))?;

    // Custom section icons (replace emoji in the README headings).
    for (file, icon) in [
        ("icon-projects.svg", "flask"),
        ("icon-companies.svg", "building"),
        ("icon-pulse.svg", "pulse"),
        ("icon-now.svg", "broadcast"),
        ("icon-contact.svg", "chat"),
    ] {
        write(file, &section_icon(icon))?;
    }

    // Contact chips as shell commands (README wraps each in a link).
    for (file, command, icon) in [
        ("contact-web.svg", "open notsu.io", "globe"),
        ("contact-devpon.svg", "open devpon.com", "layers"),
        ("contact-linkedin.svg", "open in/notsu", "linkedin"),
        ("contact-x.svg", "open x/@notsu", "x"),
        ("contact-github.svg", "gh notsu", "github"),
    ] {
        write(file, &svg::contact::button(command, icon))?;
    }

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
