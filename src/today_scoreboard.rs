use crate::utils::*;
use serde_derive::Deserialize;

// TODO: remove allocations using custom de impl
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(bound(deserialize = "'de: 'lf"))]
pub struct Games<'lf> {
    num_games: u8,
    games: Vec<Game<'lf>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(bound(deserialize = "'de: 'lf"))]
struct Game<'lf> {
    season_stage_id: usize,
    season_year: &'lf str,
    league_name: &'lf str,
    game_id: &'lf str,
    arena: Arena<'lf>,
    is_game_activated: bool,
    status_num: usize,
    extended_status_num: usize,
    start_time_eastern: &'lf str,
    #[serde(rename(deserialize = "startTimeUTC"))]
    start_time_utc: &'lf str,
    start_date_eastern: &'lf str,
    home_start_date: &'lf str,
    home_start_time: &'lf str,
    visitor_start_date: &'lf str,
    visitor_start_time: &'lf str,
    game_url_code: &'lf str,
    clock: &'lf str,
    is_buzzer_beater: bool,
    is_preview_article_avail: bool,
    is_recap_article_avail: bool,
    nugget: Nugget<'lf>,
    attendance: &'lf str,
    tickets: Tickets<'lf>,
    has_game_book_pdf: bool,
    #[serde(rename(deserialize = "isStartTimeTBD"))]
    is_start_time_tbd: bool,
    is_neutral_venue: bool,
    game_duration: GameDuration<'lf>,
    tags: Vec<&'lf str>,
    playoffs: Playoffs<'lf>,
    period: Period,
    v_team: Team<'lf>,
    h_team: Team<'lf>,
    watch: Watch<'lf>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(bound(deserialize = "'de: 'lf"))]
struct Arena<'lf> {
    name: &'lf str,
    is_domestic: bool,
    city: &'lf str,
    state_abbr: &'lf str,
    country: &'lf str,
}
