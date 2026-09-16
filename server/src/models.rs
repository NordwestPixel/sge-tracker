use chrono::{DateTime, Utc};

pub struct Team {
    pub team_id: i32,
    pub team_name: String,
    pub short_name: String,
    pub team_icon_url: String,
}

pub struct Match {
    pub match_id: i32,
    pub match_datetime: DateTime<Utc>,
    pub league_id: i32,
    pub home_id: i32,
    pub away_id: i32,
    pub last_updated: DateTime<Utc>,
    pub is_finished: bool,
    pub result_id: Vec<Result>,
}

pub struct Result {
    pub result_id: i32,
    pub result_name: String,
    pub home_goals: i32,
    pub away_goals: i32,
    pub goal: Option<Vec<Goal>>
}

pub struct Goal {
    pub goal_id: i32,
    pub player_name: String,
    pub scoring_team_id: i32,
    pub is_penalty: bool,
    pub is_own_goal: bool,
    pub is_overtime: bool,
}

pub struct LeagueTable {
    pub league_id: i32,
    pub team_id: i32,
    pub position: i32,
    pub points: i32,
    pub goals: i32,
    pub won: i32,
    pub lost: i32,
    pub draw: i32,
}

pub struct League {
    pub league_id: i32,
    pub league_name: String,
    pub league_shortcut: String,
    pub league_season: i32,
}