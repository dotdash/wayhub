use std::{
    collections::HashMap,
    io::{stdout, Write},
};

use anyhow::{bail, Result};
use config::{load_config, Counter};
use directories::ProjectDirs;
use octocrab::Octocrab;
use serde::Deserialize;
use serde_json::json;
use waybar::WaybarResult;

mod config;
mod waybar;

#[derive(Debug, Deserialize)]
struct QueryResult {
    data: ResultData,
}

#[derive(Debug, Deserialize)]
struct ResultData {
    #[serde(flatten)]
    results: HashMap<String, Stats>,
}

#[derive(Debug, Deserialize)]
struct Stats {
    #[serde(rename = "issueCount")]
    issue_count: u32,
}

fn main() -> Result<()> {
    tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .build()
        .unwrap()
        .block_on(run())
}

async fn run() -> Result<()> {
    let Some(proj_dirs) = ProjectDirs::from("", "", "Wayhub") else {
        bail!("Konnte Konfigurationsverzeichnis nicht ermitteln!")
    };

    let cfg_path = proj_dirs.config_dir().join("config.toml");
    let Ok(config) = load_config(&cfg_path) else {
        bail!("Keine gültige Konfiguration unter {}", cfg_path.display())
    };

    let octocrab = Octocrab::builder()
        .personal_token(config.github_token)
        .build()?;

    let query = config
        .counters
        .iter()
        .enumerate()
        .map(|(idx, Counter { query, .. })| {
            format!(r#"q{idx}: search(type:ISSUE, query:"{query}") {{ issueCount }}"#)
        })
        .collect::<Vec<_>>()
        .join(" ");

    let queries = octocrab
        .graphql::<QueryResult>(&json!({"query": format!("{{ {query} }}")}))
        .await?
        .data;

    let counter_text = config
        .counters
        .iter()
        .enumerate()
        .filter_map(|(idx, Counter { label, .. })| {
            let count = queries.results[&format!("q{idx}")].issue_count;
            if count > 0 {
                Some(format!("{label} {count}"))
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join(" - ");

    let waybar_result = WaybarResult {
        text: &format!(
            r#"GitHub - {}"#,
            if counter_text.len() > 0 {
                counter_text.as_str()
            } else {
                "All done!"
            }
        ),
        ..WaybarResult::default()
    };

    stdout().write_all(serde_json::to_string(&waybar_result).unwrap().as_bytes())?;

    Ok(())
}
