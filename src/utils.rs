use chrono::prelude::*;
use serde_derive::Deserialize;

pub fn today() -> String {
    let local: DateTime<Local> = Local::now();
    local.format("%Y%m%d").to_string()
}

pub trait PrettyPrintGame {
    fn print_game(&self);
}

#[derive(Deserialize, Debug, Clone)]
pub struct Watch {
    broadcast: Broadcast,
}

#[derive(Deserialize, Debug, Clone)]
struct Broadcast {
    video: Video,
}

#[derive(Deserialize, Debug, Clone)]
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

#[derive(Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    seed_num: Option<String>,
    series_win: Option<String>,
    is_series_winner: Option<bool>,
    team_id: Option<String>,
    pub tri_code: Option<String>,
    win: Option<String>,
    loss: Option<String>,
    series_loss: Option<String>,
    pub score: Option<String>,
    pub linescore: Option<Vec<Score>>,
    fast_break_points: Option<String>,
    points_in_paint: Option<String>,
    biggest_lead: Option<String>,
    second_chance_points: Option<String>,
    points_off_turnovers: Option<String>,
    longest_run: Option<String>,
    pub totals: Option<Totals>,
    leaders: Option<Leaders>,
}

impl Team {
    pub fn get_linescore(&self) -> Vec<String> {
        // TODO
        //self.linescore.as_ref().unwrap().get(0).unwrap().score;
        //self.linescore.as_ref().unwrap().get(1).unwrap().score;
        //self.linescore.as_ref().unwrap().get(2).unwrap().score;
        //self.linescore.as_ref().unwrap().get(3).unwrap().score;
        vec![String::new(); 4]
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Tickets {
    mobile_app: String,
    desktop_web: String,
    mobile_web: String,
    leag_game_info: String,
    leag_tix: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GameDuration {
    hours: String,
    minutes: String,
}

#[derive(Deserialize, Debug)]
pub struct Nugget {
    text: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Period {
    pub current: usize,
    r#type: usize,
    max_regular: usize,
    is_halftime: Option<bool>,
    is_end_of_period: Option<bool>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Playoffs {
    round_num: String,
    conf_name: String,
    series_id: String,
    series_summary_text: String,
    is_series_completed: bool,
    game_num_in_series: String,
    is_if_necessary: bool,
    v_team: Team,
    h_team: Team,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Totals {
    pub points: String,
    fgm: String,
    fga: String,
    fgp: String,
    ftm: String,
    fta: String,
    ftp: String,
    tpm: String,
    tpa: String,
    tpp: String,
    off_reb: String,
    def_reb: String,
    tot_reb: String,
    assists: String,
    p_fouls: String,
    steals: String,
    turnovers: String,
    blocks: String,
    plus_minus: String,
    min: String,
    short_timeout_remaining: Option<String>,
    full_timeout_remaining: Option<String>,
    team_fouls: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Leaders {
    points: Stat,
    rebounds: Stat,
    assists: Stat,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Stat {
    value: String,
    players: Vec<Players>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Players {
    person_id: String,
    first_name: String,
    last_name: String,
    jersey: Option<String>,
    team_id: Option<String>,
    is_on_court: Option<bool>,
    points: Option<String>,
    pos: Option<String>,
    position_full: Option<String>,
    player_code: Option<String>,
    min: Option<String>,
    fgm: Option<String>,
    fga: Option<String>,
    fgp: Option<String>,
    ftm: Option<String>,
    fta: Option<String>,
    ftp: Option<String>,
    tpm: Option<String>,
    tpa: Option<String>,
    tpp: Option<String>,
    off_reb: Option<String>,
    def_reb: Option<String>,
    tot_reb: Option<String>,
    assists: Option<String>,
    p_fouls: Option<String>,
    steals: Option<String>,
    turnovers: Option<String>,
    blocks: Option<String>,
    plus_minus: Option<String>,
    dnp: Option<String>,
    sort_key: Option<SortKey>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct SortKey {
    name: usize,
    pos: usize,
    points: usize,
    min: usize,
    fgm: usize,
    fga: usize,
    fgp: usize,
    ftm: usize,
    fta: usize,
    ftp: usize,
    tpm: usize,
    tpa: usize,
    tpp: usize,
    off_reb: usize,
    def_reb: usize,
    tot_reb: usize,
    assists: usize,
    p_fouls: usize,
    steals: usize,
    turnovers: usize,
    blocks: usize,
    plus_minus: usize,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Score {
    pub score: String,
}
