use crate::utils::*;
use reqwest::blocking::Client;
use serde_derive::Deserialize;

// TODO: remove allocations using custom de impl
#[derive(Deserialize, Debug)]
#[serde(bound(deserialize = "'de: 'lf"))]
pub struct Games<'lf> {
    league: League<'lf>,
}

impl<'lf> Games<'lf> {
    pub fn new(client: &Client) -> Result<Self, reqwest::Error> {
        // let schedules = Box::leak::<'lf>(Box::new(
        //     client
        //         .get("http://data.nba.com/prod/v1/2020/schedule.json")
        //         .send()?
        //         .text()?,
        // ));
        let schedules = Box::leak::<'lf>(Box::new(
            std::fs::read_to_string(
                std::path::PathBuf::from(std::env!("CARGO_MANIFEST_DIR").to_string())
                    .join("src/schedule.json"),
            )
            .unwrap(),
        ));

        // SAFETY: should not fail if json was properly fetched
        Ok(serde_json::from_str::<Games>(&**schedules).unwrap())
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

    fn get_today(&'lf self) -> Vec<&'lf Game<'lf>> {
        let vec = self.league.standard.as_slice();
        vec.iter()
            .filter(|&x| x.start_date_eastern == today())
            .collect()
    }

    pub fn get_date_game_id(&self, date: &str) -> Vec<&str> {
        let vec = self.league.standard.as_slice();
        vec.iter()
            .filter(|&x| x.start_date_eastern == date)
            .map(|x| x.game_id)
            .collect()
    }
}
impl<'lf> Game<'lf> {
    fn print_game(&self) {}
}

#[derive(Deserialize, Debug)]
#[serde(bound(deserialize = "'de: 'lf"))]
struct League<'lf> {
    standard: Vec<Game<'lf>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(bound(deserialize = "'de: 'lf"))]
struct Game<'lf> {
    game_id: &'lf str,
    season_stage_id: usize,
    game_url_code: &'lf str,
    status_num: usize,
    extended_status_num: usize,
    #[serde(rename(deserialize = "isStartTimeTBD"))]
    is_start_time_tbd: bool,
    #[serde(rename(deserialize = "startTimeUTC"))]
    start_time_utc: &'lf str,
    start_date_eastern: &'lf str,
    is_neutral_venue: bool,
    start_time_eastern: &'lf str,
    is_buzzer_beater: bool,
    period: Period,
    playoffs: Option<Playoffs<'lf>>,
    h_team: Team<'lf>,
    v_team: Team<'lf>,
    watch: Watch<'lf>,
}
