use chrono::{DateTime, Utc};
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};

use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Subtitle {
    name: String,
    url: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Deserialize)]
struct Record {
    header: String,
    title: String,
    titleUrl: String,
    subtitles: Option<Vec<Subtitle>>,
    time: String,
    products: Vec<String>,
    activityControls: Vec<String>,
}

#[derive(Parser)]
struct Args {
    #[clap(short, long, default_value = "10")]
    number: usize,
    #[clap(short, long)]
    after: Option<String>,
    input: PathBuf,
}

fn main() {
    let args = Args::parse();
    let after = args.after.map(|t| t.parse::<DateTime<Utc>>().unwrap());
    let input = std::fs::File::open(args.input).unwrap();
    let recs: Vec<Record> = serde_json::from_reader(input).unwrap();
    let mut channels: HashMap<_, HashSet<_>> = HashMap::new();
    for rec in &recs {
        let Some(ref sub) = rec.subtitles else {
            continue;
        };
        let Some(sub0) = sub.first() else { continue };
        let time = rec.time.parse::<DateTime<Utc>>().unwrap();
        if let Some(after) = after {
            if time < after {
                continue;
            }
        }
        channels
            .entry(sub0.name.as_str())
            .or_default()
            .insert(rec.title.as_str());
    }
    let mut channels: Vec<_> = channels
        .into_iter()
        .map(|(name, titles)| (name, titles.len()))
        .collect();
    channels.sort_by_key(|&(_, count)| usize::MAX - count);
    for (i, (name, count)) in channels.into_iter().take(args.number).enumerate() {
        println!("{}. {}: {}", i + 1, name, count);
    }
}
