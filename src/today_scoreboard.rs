use crate::utils::*;
use serde_derive::Deserialize;

// TODO: remove allocations using custom de impl
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Games {
    num_games: u8,
    games: Vec<Game>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Game {
    season_stage_id: usize,
    season_year: String,
    league_name: String,
    game_id: String,
    arena: Arena,
    is_game_activated: bool,
    status_num: usize,
    extended_status_num: usize,
    start_time_eastern: String,
    #[serde(rename(deserialize = "startTimeUTC"))]
    start_time_utc: String,
    start_date_eastern: String,
    home_start_date: String,
    home_start_time: String,
    visitor_start_date: String,
    visitor_start_time: String,
    game_url_code: String,
    clock: String,
    is_buzzer_beater: bool,
    is_preview_article_avail: bool,
    is_recap_article_avail: bool,
    nugget: Nugget,
    attendance: String,
    tickets: Tickets,
    has_game_book_pdf: bool,
    #[serde(rename(deserialize = "isStartTimeTBD"))]
    is_start_time_tbd: bool,
    is_neutral_venue: bool,
    game_duration: GameDuration,
    tags: Vec<String>,
    playoffs: Playoffs,
    period: Period,
    v_team: Team,
    h_team: Team,
    watch: Watch,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Arena {
    name: String,
    is_domestic: bool,
    city: String,
    state_abbr: String,
    country: String,
}
