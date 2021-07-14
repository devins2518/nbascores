use crate::utils::*;
use reqwest::blocking::Client;
use serde::de::Deserialize;
use serde_derive::Deserialize;

// TODO: remove allocations using custom de impl
#[derive(Debug)]
pub struct Schedule<'lf> {
    games: Vec<Game<'lf>>,
}

impl<'lf> Schedule<'lf> {
    pub fn new(client: &Client) -> Result<Self, reqwest::Error> {
        let schedules = Box::leak::<'lf>(Box::new(
            client
                .get("http://data.nba.com/prod/v1/2020/schedule.json")
                .send()?
                .text()?,
        ));
        // let schedules = Box::leak::<'lf>(Box::new(
        //     std::fs::read_to_string(
        //         std::path::PathBuf::from(std::env!("CARGO_MANIFEST_DIR").to_string())
        //             .join("src/schedule.json"),
        //     )
        //     .unwrap(),
        // ));

        // SAFETY: should not fail if json was properly fetched
        Ok(serde_json::from_str::<Schedule>(&**schedules).unwrap())
    }

    pub fn get_date_game_id(&self, date: &str) -> Vec<&str> {
        let vec = self.games.as_slice();
        vec.iter()
            .filter(|&x| x.start_date_eastern == date)
            .map(|x| x.game_id)
            .collect()
    }
}

impl<'lf, 'de> Deserialize<'de> for Schedule<'lf>
where
    'de: 'lf,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(bound(deserialize = "'de: 'lf"))]
        struct Root<'lf> {
            league: League<'lf>,
        }
        #[derive(Deserialize, Debug)]
        #[serde(bound(deserialize = "'de: 'lf"))]
        struct League<'lf> {
            standard: Vec<Game<'lf>>,
        }

        let helper = Root::deserialize(deserializer)?;

        Ok(Self {
            games: helper.league.standard,
        })
    }
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
}

#[derive(Debug)]
struct Team<'lf> {
    team_id: &'lf str,
    score: u8,
    win: u8,
    loss: u8,
}

impl<'lf, 'de> Deserialize<'de> for Team<'lf>
where
    'de: 'lf,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[serde(bound(deserialize = "'de: 'lf"))]
        struct Root<'lf> {
            team_id: &'lf str,
            score: &'lf str,
            win: &'lf str,
            loss: &'lf str,
        }

        let helper = Root::deserialize(deserializer)?;

        Ok(Self {
            team_id: helper.team_id,
            score: helper.score.parse().unwrap_or(0),
            win: helper.win.parse().unwrap_or(0),
            loss: helper.loss.parse().unwrap_or(0),
        })
    }
}

#[derive(Debug)]
struct Playoffs<'lf> {
    round_num: &'lf str,
    conf_name: &'lf str,
    series_id: &'lf str,
    is_series_completed: bool,
    game_num_in_series: &'lf str,
    is_if_necessary: bool,
    v_team_seed: u8,
    h_team_seed: u8,
}

impl<'lf, 'de> Deserialize<'de> for Playoffs<'lf>
where
    'de: 'lf,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[serde(bound(deserialize = "'de: 'lf"))]
        struct Root<'lf> {
            round_num: &'lf str,
            conf_name: &'lf str,
            series_id: &'lf str,
            is_series_completed: bool,
            game_num_in_series: &'lf str,
            is_if_necessary: bool,
            v_team: Team<'lf>,
            h_team: Team<'lf>,
        }
        #[derive(Deserialize)]
        #[serde(rename_all = "camelCase")]
        #[serde(bound(deserialize = "'de: 'lf"))]
        struct Team<'lf> {
            seed_num: &'lf str,
        }

        let helper = Root::deserialize(deserializer)?;

        Ok(Self {
            round_num: helper.round_num,
            conf_name: helper.conf_name,
            series_id: helper.series_id,
            is_series_completed: helper.is_series_completed,
            game_num_in_series: helper.game_num_in_series,
            is_if_necessary: helper.is_if_necessary,
            v_team_seed: helper.v_team.seed_num.parse().unwrap_or(0),
            h_team_seed: helper.h_team.seed_num.parse().unwrap_or(0),
        })
    }
}
