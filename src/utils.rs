use chrono::prelude::*;
use serde_derive::Deserialize;

pub fn today() -> String {
    let local: DateTime<Local> = Local::now();
    local.format("%Y%m%d").to_string()
}

pub trait PrettyPrintGame {
    fn print_game(&self);
}

#[derive(Deserialize, Debug)]
#[serde(bound(deserialize = "'de: 'lf"))]
pub struct Watch<'lf> {
    broadcast: Broadcast<'lf>,
}

#[derive(Deserialize, Debug)]
#[serde(bound(deserialize = "'de: 'lf"))]
struct Broadcast<'lf> {
    video: Video<'lf>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(bound(deserialize = "'de: 'lf"))]
struct Video<'lf> {
    regional_blackout_codes: &'lf str,
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
#[serde(bound(deserialize = "'de: 'lf"))]
pub struct Team<'lf> {
    seed_num: Option<&'lf str>,
    series_win: Option<&'lf str>,
    is_series_winner: Option<bool>,
    team_id: Option<&'lf str>,
    pub tri_code: Option<&'lf str>,
    win: Option<&'lf str>,
    loss: Option<&'lf str>,
    series_loss: Option<&'lf str>,
    pub score: Option<&'lf str>,
    pub linescore: Option<Vec<Score<'lf>>>,
    fast_break_points: Option<&'lf str>,
    points_in_paint: Option<&'lf str>,
    biggest_lead: Option<&'lf str>,
    second_chance_points: Option<&'lf str>,
    points_off_turnovers: Option<&'lf str>,
    longest_run: Option<&'lf str>,
    pub totals: Option<Totals<'lf>>,
    leaders: Option<Leaders<'lf>>,
}

impl<'lf> Team<'lf> {
    pub fn get_linescore(&self) -> Vec<&'lf str> {
        // TODO jesus christ
        let mut vec = Vec::with_capacity(4);
        let scores = self.linescore.as_ref().unwrap();
        if let Some(x) = scores.get(0) {
            vec.push(x.score.clone());
        }
        if let Some(x) = scores.get(1) {
            vec.push(x.score.clone());
        }
        if let Some(x) = scores.get(2) {
            vec.push(x.score.clone());
        }
        if let Some(x) = scores.get(3) {
            vec.push(x.score.clone());
        }

        vec
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(bound(deserialize = "'de: 'lf"))]
pub struct Tickets<'lf> {
    mobile_app: &'lf str,
    desktop_web: &'lf str,
    mobile_web: &'lf str,
    leag_game_info: &'lf str,
    leag_tix: &'lf str,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(bound(deserialize = "'de: 'lf"))]
pub struct GameDuration<'lf> {
    hours: &'lf str,
    minutes: &'lf str,
}

#[derive(Deserialize, Debug)]
#[serde(bound(deserialize = "'de: 'lf"))]
pub struct Nugget<'lf> {
    text: &'lf str,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Period {
    pub current: usize,
    r#type: usize,
    max_regular: usize,
    is_halftime: Option<bool>,
    is_end_of_period: Option<bool>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(bound(deserialize = "'de: 'lf"))]
pub struct Playoffs<'lf> {
    round_num: &'lf str,
    conf_name: &'lf str,
    series_id: &'lf str,
    series_summary_text: &'lf str,
    is_series_completed: bool,
    game_num_in_series: &'lf str,
    is_if_necessary: bool,
    v_team: Team<'lf>,
    h_team: Team<'lf>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(bound(deserialize = "'de: 'lf"))]
pub struct Totals<'lf> {
    pub points: &'lf str,
    fgm: &'lf str,
    fga: &'lf str,
    fgp: &'lf str,
    ftm: &'lf str,
    fta: &'lf str,
    ftp: &'lf str,
    tpm: &'lf str,
    tpa: &'lf str,
    tpp: &'lf str,
    off_reb: &'lf str,
    def_reb: &'lf str,
    tot_reb: &'lf str,
    assists: &'lf str,
    p_fouls: &'lf str,
    steals: &'lf str,
    turnovers: &'lf str,
    blocks: &'lf str,
    plus_minus: &'lf str,
    min: &'lf str,
    short_timeout_remaining: Option<&'lf str>,
    full_timeout_remaining: Option<&'lf str>,
    team_fouls: Option<&'lf str>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(bound(deserialize = "'de: 'lf"))]
struct Leaders<'lf> {
    points: Stat<'lf>,
    rebounds: Stat<'lf>,
    assists: Stat<'lf>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(bound(deserialize = "'de: 'lf"))]
struct Stat<'lf> {
    value: &'lf str,
    players: Vec<Players<'lf>>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(bound(deserialize = "'de: 'lf"))]
pub struct Players<'lf> {
    person_id: &'lf str,
    first_name: &'lf str,
    last_name: &'lf str,
    jersey: Option<&'lf str>,
    team_id: Option<&'lf str>,
    is_on_court: Option<bool>,
    points: Option<&'lf str>,
    pos: Option<&'lf str>,
    position_full: Option<&'lf str>,
    player_code: Option<&'lf str>,
    min: Option<&'lf str>,
    fgm: Option<&'lf str>,
    fga: Option<&'lf str>,
    fgp: Option<&'lf str>,
    ftm: Option<&'lf str>,
    fta: Option<&'lf str>,
    ftp: Option<&'lf str>,
    tpm: Option<&'lf str>,
    tpa: Option<&'lf str>,
    tpp: Option<&'lf str>,
    off_reb: Option<&'lf str>,
    def_reb: Option<&'lf str>,
    tot_reb: Option<&'lf str>,
    assists: Option<&'lf str>,
    p_fouls: Option<&'lf str>,
    steals: Option<&'lf str>,
    turnovers: Option<&'lf str>,
    blocks: Option<&'lf str>,
    plus_minus: Option<&'lf str>,
    dnp: Option<&'lf str>,
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
#[serde(bound(deserialize = "'de: 'lf"))]
pub struct Score<'lf> {
    pub score: &'lf str,
}
