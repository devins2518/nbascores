use crate::utils::*;
use serde_derive::Deserialize;

// TODO: remove allocations using custom de impl
#[derive(Deserialize, Debug)]
pub struct Games {
    league: League,
}

impl Games {
    // Likely not useful, NBA has games scheduled at the end of the season which aren't that
    // useful
    #[allow(unused)]
    pub fn print_last(&self) {
        println!("{:#?}", self.league.standard.last().unwrap());
    }

    pub fn print_today(&self) {
        let vec = self.league.standard.as_slice();
        vec.iter()
            .filter(|&x| x.start_date_eastern == today())
            .for_each(|x| println!("{:#?}", x));
    }
}

#[derive(Deserialize, Debug)]
struct League {
    standard: Vec<Game>,
}

#[derive(Deserialize, Debug)]
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
    h_team: HTeam,
    v_team: VTeam,
    watch: Watch,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Period {
    current: usize,
    #[serde(rename(deserialize = "type"))]
    type_: usize,
    max_regular: usize,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Playoffs {
    round_num: String,
    conf_name: String,
    series_id: String,
    series_summary_text: String,
    is_series_completed: bool,
    game_num_in_series: String,
    is_if_necessary: bool,
    v_team: VTeam,
    h_team: HTeam,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct VTeam {
    seed_num: Option<String>,
    series_win: Option<String>,
    is_series_winner: Option<bool>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct HTeam {
    seed_num: Option<String>,
    series_win: Option<String>,
    is_series_winner: Option<bool>,
}

#[derive(Deserialize, Debug)]
struct Nugget {
    text: String,
}

#[derive(Deserialize, Debug)]
struct Watch {
    broadcast: Broadcast,
}

#[derive(Deserialize, Debug)]
struct Broadcast {
    video: Video,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Video {
    regional_blackout_codes: String,
    is_league_pass: bool,
    is_national_blackout: bool,
    #[serde(rename(deserialize = "isTNTOT"))]
    is_tntot: bool,
    can_purchase: bool,
    #[serde(rename(deserialize = "isVR"))]
    is_vr: bool,
    #[serde(rename(deserialize = "isNextVR"))]
    is_next_vr: bool,
    #[serde(rename(deserialize = "isNBAOnTNTVR"))]
    is_nba_on_tnt_vr: bool,
    is_magic_leap: bool,
    is_oculus_venues: bool,
}
