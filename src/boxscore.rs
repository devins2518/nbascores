use crate::utils::*;
use serde_derive::Deserialize;

// TODO: remove allocations using custom de impl
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BoxScore {
    pub basic_game_data: BasicGameData,
    pub previous_matchup: PreviousMatchup,
    pub stats: Option<Stats>,
}

impl BoxScore {
    pub fn new(
        client: &reqwest::blocking::Client,
        game_date: String,
        game_id: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        println!(
            "http://data.nba.com/prod/v1/{}/{}_boxscore.json",
            game_date, game_id
        );
        let boxscore = client
            .get(format!(
                "http://data.nba.com/prod/v1/{}/{}_boxscore.json",
                game_date, game_id
            ))
            .send()?
            .text()?;

        Ok(serde_json::from_str::<BoxScore>(&boxscore)?)
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BasicGameData {
    season_stage_id: usize,
    season_year: String,
    league_name: String,
    game_id: String,
    //arena:
    pub is_game_activated: bool,
    status_num: usize,
    extended_status_num: usize,
    pub start_time_eastern: String,
    #[serde(rename(deserialize = "startTimeUTC"))]
    start_time_utc: String,
    #[serde(rename(deserialize = "endTimeUTC"))]
    end_time_utc: Option<String>,
    pub start_date_eastern: String,
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
    tags: Option<Vec<String>>,
    playoffs: Option<Playoffs>,
    period: Period,
    pub v_team: Team,
    pub h_team: Team,
    //watch,
    officials: Officials,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PreviousMatchup {
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

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    times_tied: String,
    lead_changes: String,
    v_team: Team,
    h_team: Team,
    active_players: Vec<Players>,
}

impl Stats {
    pub fn get_total_v(&self) -> String {
        if let Some(total) = &self.v_team.totals {
            total.points.clone()
        } else {
            String::from("0")
        }
    }
    pub fn get_total_h(&self) -> String {
        if let Some(total) = &self.h_team.totals {
            total.points.clone()
        } else {
            String::from("0")
        }
    }
}

impl PrettyPrintGame for BoxScore {
    fn print_game(&self) {
        let bgd = &self.basic_game_data;
        let stats = self.stats.as_ref();
        let v_linescore = bgd.v_team.get_linescore();
        let h_linescore = bgd.h_team.get_linescore();

        println!(" T      1  2  3  4  T");
        //if bgd.period.current == 0 {
        //} else {
        // TODO crashes if run for todays game that hasnt started
        println!(
            "{}    {: >2} {: >2} {: >2} {: >2} {: >3}",
            bgd.v_team.tri_code.as_ref().unwrap(),
            v_linescore[0],
            v_linescore[1],
            v_linescore[2],
            v_linescore[3],
            stats.unwrap_or(&Stats::default()).get_total_v()
        );
        println!(
            "{}    {: >2} {: >2} {: >2} {: >2} {: >3}",
            bgd.h_team.tri_code.as_ref().unwrap(),
            h_linescore[0],
            h_linescore[1],
            h_linescore[2],
            h_linescore[3],
            stats.unwrap_or(&Stats::default()).get_total_h()
        )
        //}
    }
}
