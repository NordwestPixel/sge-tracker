use chrono::{DateTime, NaiveDateTime, Utc};

pub(crate) struct MatchRow {
    pub(crate) match_id: i32,
    pub(crate) league_id: i32,
    pub(crate) matchday: i32,
    pub(crate) matchday_name: String,
    pub(crate) home_team_id: i32,
    pub(crate) away_team_id: i32,
    pub(crate) kickoff_at: DateTime<Utc>,
    pub(crate) is_finished: bool,
    pub(crate) source_updated_at: Option<NaiveDateTime>,
}

pub(crate) struct ResultRow {
    pub(crate) match_id: i32,
    pub(crate) result_type_id: i32,
    pub(crate) home_goals: i32,
    pub(crate) away_goals: i32,
}

pub(crate) struct GoalRow {
    pub(crate) goal_id: i32,
    pub(crate) match_id: i32,
    pub(crate) home_score: i32,
    pub(crate) away_score: i32,
    pub(crate) minute: Option<i32>,
    pub(crate) player_name: Option<String>,
    pub(crate) scoring_team_id: Option<i32>,
    pub(crate) is_penalty: bool,
    pub(crate) is_own_goal: bool,
    pub(crate) is_overtime: bool,
}
