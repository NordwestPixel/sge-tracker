pub(crate) struct TeamRow {
    pub(crate) team_id: i32,
    pub(crate) name: String,
    pub(crate) short_name: Option<String>,
    pub(crate) icon_url: Option<String>,
}

pub(crate) struct LeagueRow {
    pub(crate) league_id: i32,
    pub(crate) name: String,
    pub(crate) shortcut: String,
    pub(crate) season: i32,
}
