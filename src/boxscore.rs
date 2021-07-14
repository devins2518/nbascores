use crate::utils::*;
use serde::de::Deserialize;
use serde_derive::Deserialize;

#[derive(Debug)]
pub struct BoxScore<'lf> {
    pub players: Vec<Player<'lf>>,
    pub v_team: Team<'lf>,
    pub h_team: Team<'lf>,
}

impl<'lf> BoxScore<'lf> {
    pub fn new(
        client: &reqwest::blocking::Client,
        game_date: &str,
        game_id: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let boxscore = Box::leak::<'lf>(Box::new(
            client
                .get(format!(
                    "http://data.nba.com/prod/v1/{}/{}_boxscore.json",
                    game_date, game_id
                ))
                .send()?
                .text()?,
        ));
        // let boxscore = Box::leak::<'lf>(Box::new(
        //     std::fs::read_to_string(
        //         std::path::PathBuf::from(format!("{}", std::env!("CARGO_MANIFEST_DIR")))
        //             .join("src/boxscore.json"),
        //     )
        //     .unwrap(),
        // ));

        let boxscore = serde_json::from_str::<BoxScore<'lf>>(boxscore)?;

        Ok(boxscore)
    }
}

impl<'lf, 'de> Deserialize<'de> for BoxScore<'lf>
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
            #[serde(rename(deserialize = "basicGameData"))]
            bgd: BasicGameData<'lf>,
            stats: Option<Stats<'lf>>,
        }
        let helper = Root::deserialize(deserializer)?;
        let players = if let Some(x) = helper.stats {
            x.active_players
        } else {
            [
                Player::from_team_id(helper.bgd.h_team.team_id.unwrap()),
                Player::from_team_id(helper.bgd.v_team.team_id.unwrap()),
            ]
            .concat()
        };

        Ok(Self {
            players,
            v_team: helper.bgd.v_team,
            h_team: helper.bgd.h_team,
        })
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(bound(deserialize = "'de: 'lf"))]
pub struct BasicGameData<'lf> {
    season_stage_id: usize,
    season_year: &'lf str,
    league_name: &'lf str,
    game_id: &'lf str,
    //arena:
    pub is_game_activated: bool,
    status_num: usize,
    extended_status_num: usize,
    pub start_time_eastern: &'lf str,
    #[serde(rename(deserialize = "startTimeUTC"))]
    start_time_utc: &'lf str,
    #[serde(rename(deserialize = "endTimeUTC"))]
    end_time_utc: Option<&'lf str>,
    pub start_date_eastern: &'lf str,
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
    tags: Option<Vec<&'lf str>>,
    playoffs: Option<Playoffs<'lf>>,
    period: Period,
    pub v_team: Team<'lf>,
    pub h_team: Team<'lf>,
    //watch,
    officials: Officials<'lf>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(bound(deserialize = "'de: 'lf"))]
pub struct PreviousMatchup<'lf> {
    game_id: &'lf str,
    game_date: &'lf str,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(bound(deserialize = "'de: 'lf"))]
struct Officials<'lf> {
    formatted: Vec<Name<'lf>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(bound(deserialize = "'de: 'lf"))]
struct Name<'lf> {
    first_name_last_name: &'lf str,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(bound(deserialize = "'de: 'lf"))]
pub struct Stats<'lf> {
    times_tied: &'lf str,
    lead_changes: &'lf str,
    v_team: Team<'lf>,
    h_team: Team<'lf>,
    pub active_players: Vec<Player<'lf>>,
}

impl<'lf> Default for Stats<'lf> {
    fn default() -> Self {
        Self {
            times_tied: "0",
            lead_changes: "0",
            v_team: Team::default(),
            h_team: Team::default(),
            active_players: Vec::new(),
        }
    }
}
