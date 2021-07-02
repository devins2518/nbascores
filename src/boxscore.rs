use crate::utils::*;
use serde_derive::Deserialize;

// TODO: remove allocations using custom de impl
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(bound(deserialize = "'de: 'lf"))]
pub struct BoxScore<'lf> {
    pub basic_game_data: BasicGameData<'lf>,
    pub previous_matchup: PreviousMatchup<'lf>,
    pub stats: Option<Stats<'lf>>,
}

impl<'lf> BoxScore<'lf> {
    pub fn new(
        client: &reqwest::blocking::Client,
        game_date: String,
        game_id: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        println!(
            "http://data.nba.com/prod/v1/{}/{}_boxscore.json",
            game_date, game_id
        );
        let boxscore = Box::leak(Box::new(
            client
                .get(format!(
                    "http://data.nba.com/prod/v1/{}/{}_boxscore.json",
                    game_date, game_id
                ))
                .send()?
                .text()?,
        ));

        Ok(serde_json::from_str::<BoxScore<'lf>>(&*boxscore)?)
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
    active_players: Vec<Players<'lf>>,
}

impl<'lf> Stats<'lf> {
    pub fn get_total_v(&self) -> &str {
        if let Some(total) = &self.v_team.totals {
            total.points
        } else {
            "0"
        }
    }
    pub fn get_total_h(&self) -> &str {
        if let Some(total) = &self.h_team.totals {
            total.points
        } else {
            "0"
        }
    }
}

impl<'lf> PrettyPrintGame for BoxScore<'lf> {
    fn print_game(&self) {
        let bgd = &self.basic_game_data;
        let stats = if let Some(ref stats) = self.stats {
            stats.clone()
        } else {
            Stats {
                times_tied: "0",
                lead_changes: "0",
                v_team: Team::default(),
                h_team: Team::default(),
                active_players: Vec::new(),
            }
        };
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
            stats.get_total_v()
        );
        println!(
            "{}    {: >2} {: >2} {: >2} {: >2} {: >3}",
            bgd.h_team.tri_code.as_ref().unwrap(),
            h_linescore[0],
            h_linescore[1],
            h_linescore[2],
            h_linescore[3],
            stats.get_total_h()
        )
        //}
    }
}
