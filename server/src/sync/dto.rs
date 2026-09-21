use chrono::{DateTime, NaiveDateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchResponse {
    #[serde(rename = "matchID")]
    pub match_id: i32,
    #[serde(rename = "matchDateTimeUTC")]
    pub match_date_time_utc: DateTime<Utc>,
    #[serde(rename = "timeZoneID")]
    pub time_zone_id: Option<String>,
    pub league_id: i32,
    pub league_name: String,
    pub league_season: i32,
    pub league_shortcut: String,
    pub group: GroupResponse,
    pub team1: TeamResponse,
    pub team2: TeamResponse,
    pub last_update_date_time: Option<NaiveDateTime>,
    pub match_is_finished: bool,
    pub match_results: Vec<MatchResultResponse>,
    pub goals: Vec<GoalResponse>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GroupResponse {
    pub group_name: String,
    #[serde(rename = "groupOrderID")]
    pub group_order_id: i32,
    #[serde(rename = "groupID")]
    pub group_id: i32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamResponse {
    pub team_id: i32,
    pub team_name: String,
    pub short_name: String,
    pub team_icon_url: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MatchResultResponse {
    #[serde(rename = "resultTypeID")]
    pub result_type_id: i32,
    pub points_team1: i32,
    pub points_team2: i32,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GoalResponse {
    #[serde(rename = "goalID")]
    pub goal_id: i32,
    pub score_team1: i32,
    pub score_team2: i32,
    pub match_minute: Option<i32>,
    pub goal_getter_name: String,
    pub scoring_team_id: Option<i32>,
    pub is_penalty: bool,
    pub is_own_goal: bool,
    pub is_overtime: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StandingsResponse {
    pub team_info_id: i32,
    pub points: i32,
    pub goals: i32,
    pub opponent_goals: i32,
    pub won: i32,
    pub lost: i32,
    pub draw: i32,
}