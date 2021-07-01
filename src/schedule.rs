use crate::utils::*;
use reqwest::blocking::Client;
use serde_derive::Deserialize;

// TODO: remove allocations using custom de impl
#[derive(Deserialize, Debug)]
pub struct Games {
    league: League,
}

impl Games {
    pub fn new(client: &Client) -> Result<Self, Box<dyn std::error::Error>> {
        let schedules = client
            .get("http://data.nba.com/prod/v1/2020/schedule.json")
            .send()?
            .text()?;

        Ok(serde_json::from_str::<Games>(&schedules)?)
    }

    // Likely not useful, NBA has games scheduled at the end of the season which aren't that
    // useful
    #[allow(unused)]
    pub fn print_last(&self) {
        println!("{:#?}", self.league.standard.last().unwrap());
    }

    pub fn print_today(&self) {
        self.get_today().iter().for_each(|x| x.print_game());
    }

    fn get_today(&self) -> Vec<Game> {
        let vec = self.league.standard.as_slice();
        vec.iter()
            .filter(|&x| x.start_date_eastern == today())
            .map(|x| x.clone())
            .collect()
    }

    pub fn get_date_game_id(&self, game_id: String) -> Vec<String> {
        let vec = self.league.standard.as_slice();
        vec.iter()
            .filter(|&x| x.start_date_eastern == game_id)
            .map(|x| x.game_id.clone())
            .collect()
    }
}

#[derive(Deserialize, Debug)]
struct League {
    standard: Vec<Game>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Game {
    game_id: String,
    season_stage_id: usize,
    game_url_code: String,
    status_num: usize,
    extended_status_num: usize,
    #[serde(rename(deserialize = "isStartTimeTBD"))]
    is_start_time_tbd: bool,
    #[serde(rename(deserialize = "startTimeUTC"))]
    start_time_utc: String,
    start_date_eastern: String,
    is_neutral_venue: bool,
    start_time_eastern: String,
    is_buzzer_beater: bool,
    period: Period,
    playoffs: Option<Playoffs>,
    h_team: Team,
    v_team: Team,
    watch: Watch,
}

impl PrettyPrintGame for Game {
    // TODO
    fn print_game(&self) {}
}
