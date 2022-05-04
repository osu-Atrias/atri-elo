use std::collections::HashSet;

use chrono::{DateTime, Utc};

pub struct ContestSeries {
    id: u64,
    name: String,
    desc: String,
    creator: String,
    contest_ids: HashSet<u64>
}

pub enum ContestStatus {
    Planned {
        start_time: DateTime<Utc>
    },
    Open {
        start_time: DateTime<Utc>
    },
    Closed {
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>
    }
}

pub struct Contest {
    id: u64,
    series_id: u64,
    name: String,
    desc: String,
    creator: String,
    status: ContestStatus,
    match_ids: HashSet<u64>,
}

pub enum MatchStatus {
    Planned {
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>
    },
    Open {
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>
    },
    Closed {
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>
    },
    Rated {
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        rate_time: DateTime<Utc>,
    }
}

pub enum MatchMetric {
    Score,
    Combo,
    Accuracy,
    Nmiss,
}

pub struct Match {
    id: u64,
    contest_id: u64,
    beatmap_id: u64,
    beatmap_mods: u32,
    beatmap_mode: u8,
    status: MatchStatus,
    entries: HashSet<u64>
}

pub struct Entry {
    id: u64,
    match_id: u64,
    score_id: u64,
    player_id: u64,
    score: u64,
    pp: f64,
    combo: u64,
    n300: u64,
    n100: u64,
    n50: u64,
    nmiss: u64,
    ngeki: u64,
    nkatu: u64,
    mods: u32,
    mode: u8,
    play_time: DateTime<Utc>,
    elo_perf: Option<f64>,
    elo_diff: Option<f64>
}