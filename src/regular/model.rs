use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Json;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ContestStatus {
    Planned {
        start_time: DateTime<Utc>,
    },
    Open {
        start_time: DateTime<Utc>,
    },
    Closed {
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    },
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MatchStatus {
    Planned {
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    },
    Open {
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    },
    Closed {
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    },
    Rating {
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
    },
    Rated {
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        rate_time: DateTime<Utc>,
    },
}

pub struct ContestGroup {
    id: i64,
    name: String,
    description: String,
}

pub struct Contest {
    id: i64,
    group_id: i64,
    name: String,
    description: String,
    creator: String,
    status: Json<ContestStatus>,
}

#[derive(sqlx::Type, Debug, Clone, Copy)]
#[sqlx(type_name = "sorting_metric", rename_all = "lowercase")]
pub enum SortingMetric {
    Score,
    Acc,
    Combo,
    Nmiss,
}

pub struct Match {
    id: i64,
    contest_id: i64,
    beatmap_id: i64,
    beatmap_mods: i32,
    beatmap_mode: i16,
    major_metric: SortingMetric,
    minor_metric: SortingMetric,
    status: Json<MatchStatus>,
}

pub struct Entry {
    id: i64,
    match_id: i64,
    score_id: i64,
    score: i32,
    pp: f64,
    combo: i32,
    n300: i32,
    n100: i32,
    n50: i32,
    nmiss: i32,
    ngeki: i32,
    nkatu: i32,
    mods: i32,
    mode: i16,
    play_time: DateTime<Utc>,
    player_id: i64,
    elo_perf: Option<f64>,
    elo_diff: Option<f64>,
}
