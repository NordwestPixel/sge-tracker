use crate::matches::model::{GoalRow, MatchRow, ResultRow};
use crate::reference::model::{LeagueRow, TeamRow};
use crate::standings::model::StandingRow;
use crate::sync::dto::{
    GoalResponse, MatchResponse, MatchResultResponse, StandingsResponse, TeamResponse,
};
use std::collections::BTreeMap;

fn trimmed(value: String) -> String {
    value.trim().to_owned()
}

fn non_empty(value: String) -> Option<String> {
    let value = trimmed(value);
    (!value.is_empty()).then_some(value)
}

impl From<TeamResponse> for TeamRow {
    fn from(response: TeamResponse) -> Self {
        Self {
            team_id: response.team_id,
            name: trimmed(response.team_name),
            short_name: non_empty(response.short_name),
            icon_url: non_empty(response.team_icon_url),
        }
    }
}

fn result_row(match_id: i32, response: MatchResultResponse) -> ResultRow {
    ResultRow {
        match_id,
        result_type_id: response.result_type_id,
        home_goals: response.points_team1,
        away_goals: response.points_team2,
    }
}

fn goal_row(match_id: i32, response: GoalResponse) -> GoalRow {
    GoalRow {
        goal_id: response.goal_id,
        match_id,
        home_score: response.score_team1,
        away_score: response.score_team2,
        minute: response.match_minute,
        player_name: non_empty(response.goal_getter_name),
        scoring_team_id: response.scoring_team_id,
        is_penalty: response.is_penalty,
        is_own_goal: response.is_own_goal,
        is_overtime: response.is_overtime,
    }
}

pub(super) struct MappedMatches {
    pub(super) league: Option<LeagueRow>,
    pub(super) teams: Vec<TeamRow>,
    pub(super) matches: Vec<MatchRow>,
    pub(super) results: Vec<ResultRow>,
    pub(super) goals: Vec<GoalRow>,
}

pub(super) fn map_matches(responses: Vec<MatchResponse>) -> MappedMatches {
    let mut league = None;
    let mut teams = BTreeMap::new();
    let mut matches = Vec::new();
    let mut results = Vec::new();
    let mut goals = Vec::new();

    for response in responses {
        if league.is_none() {
            league = Some(LeagueRow {
                league_id: response.league_id,
                name: trimmed(response.league_name),
                shortcut: trimmed(response.league_shortcut),
                season: response.league_season,
            });
        }

        matches.push(MatchRow {
            match_id: response.match_id,
            league_id: response.league_id,
            matchday: response.group.group_order_id,
            matchday_name: trimmed(response.group.group_name),
            home_team_id: response.team1.team_id,
            away_team_id: response.team2.team_id,
            kickoff_at: response.match_date_time_utc,
            is_finished: response.match_is_finished,
            source_updated_at: response.last_update_date_time,
        });

        teams.insert(response.team1.team_id, response.team1.into());
        teams.insert(response.team2.team_id, response.team2.into());

        results.extend(
            response
                .match_results
                .into_iter()
                .map(|result| result_row(response.match_id, result)),
        );

        goals.extend(
            response
                .goals
                .into_iter()
                .map(|goal| goal_row(response.match_id, goal)),
        );
    }

    MappedMatches {
        league,
        teams: teams.into_values().collect(),
        matches,
        results,
        goals,
    }
}

pub(super) fn map_standings(league_id: i32, responses: Vec<StandingsResponse>) -> Vec<StandingRow> {
    responses
        .into_iter()
        .enumerate()
        .map(|(index, standing)| StandingRow {
            league_id,
            team_id: standing.team_info_id,
            position: i32::try_from(index + 1).expect("table position should fit in an i32"),
            points: standing.points,
            won: standing.won,
            draw: standing.draw,
            lost: standing.lost,
            goals_for: standing.goals,
            goals_against: standing.opponent_goals,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cup_matches() -> MappedMatches {
        let json = include_str!("../../tests/fixtures/dfb_2026_matches.json");
        map_matches(serde_json::from_str(json).unwrap())
    }

    fn bundesliga_table() -> Vec<StandingRow> {
        let json = include_str!("../../tests/fixtures/bl1_2026_table.json");
        map_standings(4937, serde_json::from_str(json).unwrap())
    }

    #[test]
    fn maps_every_match_result_and_goal() {
        let mapped = cup_matches();

        assert_eq!(mapped.matches.len(), 48);
        assert_eq!(mapped.results.len(), 74);
        assert_eq!(mapped.goals.len(), 177);
    }

    #[test]
    fn deduplicates_teams() {
        assert_eq!(cup_matches().teams.len(), 64);
    }

    #[test]
    fn takes_league_from_match_data() {
        let league = cup_matches().league.unwrap();

        assert_eq!(league.league_id, 4945);
        assert_eq!(league.name, "DFB Pokal 2026/2027");
        assert_eq!(league.shortcut, "dfb");
        assert_eq!(league.season, 2026);
    }

    #[test]
    fn empty_input_has_no_league() {
        let mapped = map_matches(Vec::new());

        assert!(mapped.league.is_none());
        assert!(mapped.teams.is_empty());
        assert!(mapped.matches.is_empty());
    }

    #[test]
    fn team1_is_the_home_team() {
        let mapped = cup_matches();
        let row = mapped.matches.iter().find(|m| m.match_id == 81845).unwrap();

        assert_eq!(row.home_team_id, 7594);
        assert_eq!(row.away_team_id, 81);
    }

    #[test]
    fn trims_padded_team_names() {
        let mapped = cup_matches();
        let team = mapped.teams.iter().find(|t| t.team_id == 7594).unwrap();

        assert_eq!(team.name, "VfB 1921 Krieschow");
    }

    #[test]
    fn empty_short_name_becomes_none() {
        let mapped = cup_matches();
        let team = mapped.teams.iter().find(|t| t.team_id == 1071).unwrap();

        assert_eq!(team.short_name, None);
    }

    #[test]
    fn keeps_missing_goal_minute_as_none() {
        let mapped = cup_matches();
        let goal = mapped.goals.iter().find(|g| g.goal_id == 145815).unwrap();

        assert_eq!(goal.minute, None);
        assert_eq!(goal.player_name.as_deref(), Some("Linus Gechter"));
    }

    #[test]
    fn standings_positions_follow_list_order() {
        let table = bundesliga_table();
        let positions: Vec<i32> = table.iter().map(|row| row.position).collect();

        assert_eq!(positions, (1..=18).collect::<Vec<i32>>());
        assert_eq!(table[0].team_id, 7);
        assert_eq!(table[17].team_id, 87);
    }

    #[test]
    fn standings_carry_the_league_id() {
        assert!(bundesliga_table().iter().all(|row| row.league_id == 4937));
    }
}
