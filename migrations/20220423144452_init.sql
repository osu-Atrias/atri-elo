CREATE SCHEMA regular;

CREATE TABLE regular.contest_groups (
    id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name text NOT NULL UNIQUE,
    description text NOT NULL
);

CREATE TABLE regular.contests (
    id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    group_id bigint NOT NULL REFERENCES regular.contest_groups,
    name text NOT NULL UNIQUE,
    description text NOT NULL,
    creator text NOT NULL,
    status jsonb NOT NULL
);

CREATE TYPE sorting_metric AS ENUM (
    'score',
    'acc',
    'combo',
    'nmiss'
);

CREATE TABLE regular.matches (
    id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    contest_id bigint NOT NULL REFERENCES regular.contests,
    beatmap_id bigint NOT NULL,
    beatmap_mods integer NOT NULL,
    beatmap_mode tinyint NOT NULL,
    major_metric sorting_metric NOT NULL,
    minor_metric sorting_metric NOT NULL,
    status jsonb NOT NULL
);

CREATE TABLE regular.entries (
    id bigint PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    match_id bigint NOT NULL REFERENCES regular.matches,
    score_id bigint NOT NULL,
    score integer NOT NULL,
    pp double precision NOT NULL,
    acc double precision NOT NULL,
    combo integer NOT NULL,
    n300 integer NOT NULL,
    n100 integer NOT NULL,
    n50 integer NOT NULL,
    nmiss integer NOT NULL,
    ngeki integer NOT NULL,
    nkatu integer NOT NULL,
    mods integer NOT NULL,
    mode tinyint NOT NULL,
    play_time timestamptz NOT NULL,
    player_id bigint NOT NULL REFERENCES regular.players,
    elo_perf double precision NULL,
    elo_diff double precision NULL
);

CREATE TABLE regular.players (
    id bigint PRIMARY KEY,
    name text NOT NULL UNIQUE,
    mu double precision NOT NULL,
    mu_pi double precision NOT NULL,
    sigma double precision NOT NULL,
    delta double precision NOT NULL,
    perfs double precision[] NOT NULL,
    weights double precision[] NOT NULL
);

