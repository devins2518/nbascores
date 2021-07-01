use crate::utils::*;
use serde_derive::Deserialize;

// TODO: remove allocations using custom de impl
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BoxScore {
    basic_game_data: BasicGameData,
    previous_matchup: PreviousMatchup,
    stats: Stats,
}

impl BoxScore {
    pub fn new(
        client: &reqwest::blocking::Client,
        game_date: String,
        game_id: String,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        println!("game_date: {}, game_id {}", game_date, game_id);
        let schedules = client
            .get(format!(
                "http://data.nba.com/prod/v1/{}/{}_boxscore.json",
                game_date, game_id
            ))
            .send()?
            .text()?;

        Ok(serde_json::from_str::<BoxScore>(&schedules)?)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct BasicGameData {
    season_stage_id: usize,
    season_year: String,
    league_name: String,
    game_id: String,
    //arena:
    is_game_activated: bool,
    status_num: usize,
    extended_status_num: usize,
    start_time_eastern: String,
    #[serde(rename(deserialize = "startTimeUTC"))]
    start_time_utc: String,
    #[serde(rename(deserialize = "endTimeUTC"))]
    end_time_utc: Option<String>,
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
    //watch,
    officials: Officials,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct PreviousMatchup {
    game_id: String,
    game_date: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Officials {
    formatted: Vec<Name>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Name {
    first_name_last_name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Stats {
    times_tied: String,
    lead_changes: String,
    v_team: Team,
    h_team: Team,
    active_players: Vec<Players>,
}
