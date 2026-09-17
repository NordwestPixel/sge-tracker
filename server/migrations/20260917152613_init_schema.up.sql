CREATE TABLE teams
(
    team_id    INTEGER PRIMARY KEY,
    name       TEXT NOT NULL,
    short_name TEXT,
    icon_url   TEXT
);

CREATE TABLE leagues
(
    league_id INTEGER PRIMARY KEY,
    name      TEXT    NOT NULL,
    shortcut  TEXT    NOT NULL,
    season    INTEGER NOT NULL,
    UNIQUE (shortcut, season)
);

CREATE TABLE matches
(
    match_id          INTEGER PRIMARY KEY,
    league_id         INTEGER REFERENCES leagues NOT NULL,
    matchday          INTEGER                    NOT NULL,
    matchday_name     TEXT                       NOT NULL,
    home_team_id      INTEGER REFERENCES teams   NOT NULL,
    away_team_id      INTEGER REFERENCES teams   NOT NULL,
    kickoff_at        TIMESTAMPTZ                NOT NULL,
    is_finished       BOOLEAN                    NOT NULL,
    source_updated_at TIMESTAMP,
    CHECK ( home_team_id <> away_team_id )
);

CREATE TABLE result_types
(
    result_type_id INTEGER PRIMARY KEY,
    name           TEXT NOT NULL
);

INSERT INTO result_types (result_type_id, name)
VALUES (1, 'half-time'),
       (2, 'after 90 min'),
       (3, 'after stoppage time'),
       (4, 'after extra time'),
       (5, 'after penalties');

CREATE TABLE match_results
(
    match_id       INTEGER REFERENCES matches ON DELETE CASCADE,
    result_type_id INTEGER REFERENCES result_types,
    home_goals     INTEGER NOT NULL,
    away_goals     INTEGER NOT NULL,
    CHECK ( home_goals >= 0 ),
    CHECK ( away_goals >= 0 ),
    PRIMARY KEY (match_id, result_type_id)
);

CREATE TABLE goals
(
    goal_id         INTEGER PRIMARY KEY,
    match_id        INTEGER REFERENCES matches ON DELETE CASCADE NOT NULL,
    home_score      INTEGER                                      NOT NULL,
    away_score      INTEGER                                      NOT NULL,
    minute          INTEGER,
    player_name     TEXT,
    scoring_team_id INTEGER REFERENCES teams,
    is_penalty      BOOLEAN                                      NOT NULL,
    is_own_goal     BOOLEAN                                      NOT NULL,
    is_overtime     BOOLEAN                                      NOT NULL,
    CHECK ( home_score >= 0 ),
    CHECK ( away_score >= 0 ),
    CHECK ( minute >= 0
) );

CREATE TABLE standings
(
    league_id     INTEGER REFERENCES leagues,
    team_id       INTEGER REFERENCES teams,
    position      INTEGER NOT NULL,
    points        INTEGER NOT NULL,
    won           INTEGER NOT NULL,
    draw          INTEGER NOT NULL,
    lost          INTEGER NOT NULL,
    goals_for     INTEGER NOT NULL,
    goals_against INTEGER NOT NULL,
    CHECK ( position >= 1 ),
    CHECK ( won >= 0 ),
    CHECK ( draw >= 0 ),
    CHECK ( lost >= 0 ),
    CHECK ( goals_for >= 0 ),
    CHECK ( goals_against >= 0 ),
    PRIMARY KEY (league_id, team_id),
    UNIQUE (league_id, position)
);

CREATE INDEX matches_kickoff_idx ON matches (kickoff_at);
CREATE INDEX matches_home_idx ON matches (home_team_id);
CREATE INDEX matches_away_idx ON matches (away_team_id);
CREATE INDEX matches_league_idx ON matches (league_id);

CREATE INDEX goals_match_idx ON goals (match_id);
