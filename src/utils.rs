use chrono::prelude::*;
use serde::de::Deserialize;
use serde_derive::Deserialize;
use tui::widgets::ListState;

pub fn today() -> String {
    let local: DateTime<Local> = Local::now();
    local.format("%Y%m%d").to_string()
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

#[derive(Deserialize, Debug)]
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
    players: Vec<Player<'lf>>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(bound(deserialize = "'de: 'lf"))]
pub struct Player<'lf> {
    person_id: &'lf str,
    pub first_name: &'lf str,
    pub last_name: &'lf str,
    jersey: Option<&'lf str>,
    pub team_id: Option<&'lf str>,
    pub is_on_court: Option<bool>,
    pub points: Option<&'lf str>,
    pub pos: Option<&'lf str>,
    position_full: Option<&'lf str>,
    player_code: Option<&'lf str>,
    pub min: Option<&'lf str>,
    pub fgm: Option<&'lf str>,
    pub fga: Option<&'lf str>,
    pub fgp: Option<&'lf str>,
    pub ftm: Option<&'lf str>,
    pub fta: Option<&'lf str>,
    pub ftp: Option<&'lf str>,
    pub tpm: Option<&'lf str>,
    pub tpa: Option<&'lf str>,
    pub tpp: Option<&'lf str>,
    pub off_reb: Option<&'lf str>,
    pub def_reb: Option<&'lf str>,
    pub tot_reb: Option<&'lf str>,
    pub assists: Option<&'lf str>,
    pub p_fouls: Option<&'lf str>,
    pub steals: Option<&'lf str>,
    pub turnovers: Option<&'lf str>,
    pub blocks: Option<&'lf str>,
    pub plus_minus: Option<&'lf str>,
    pub dnp: Option<&'lf str>,
    pub sort_key: Option<SortKey>,
}

impl<'lf> Player<'lf> {
    pub fn from_team_id(team_id: &str) -> Vec<Self> {
        let json = Box::leak::<'lf>(Box::new(
            reqwest::blocking::get("http://data.nba.com/10s//prod/v1/2020/players.json")
                .expect("Could not fetch roster json")
                .text()
                .unwrap(),
        ));

        let roster = serde_json::from_str::<Roster<'lf>>(json).unwrap();
        roster
            .players
            .into_iter()
            .filter(|x| x.team_id.unwrap() == team_id)
            .collect()
    }
}

struct Roster<'lf> {
    players: Vec<Player<'lf>>,
}

impl<'lf, 'de> Deserialize<'de> for Roster<'lf>
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
            league: Standard<'lf>,
        }

        #[derive(Deserialize)]
        #[serde(bound(deserialize = "'de: 'lf"))]
        struct Standard<'lf> {
            #[serde(rename(deserialize = "standard"))]
            players: Vec<Player<'lf>>,
        }

        let helper = Root::deserialize(deserializer)?;

        Ok(Self {
            players: helper.league.players,
        })
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SortKey {
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

#[derive(Clone)]
pub struct RandomSignal {
    value: u64,
}

impl RandomSignal {
    pub fn new() -> RandomSignal {
        RandomSignal { value: 50 }
    }
}

impl Iterator for RandomSignal {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        Some(50)
    }
}

#[derive(Clone)]
pub struct SinSignal {
    x: f64,
    interval: f64,
    period: f64,
    scale: f64,
}

impl SinSignal {
    pub fn new(interval: f64, period: f64, scale: f64) -> SinSignal {
        SinSignal {
            x: 0.0,
            interval,
            period,
            scale,
        }
    }
}

impl Iterator for SinSignal {
    type Item = (f64, f64);
    fn next(&mut self) -> Option<Self::Item> {
        let point = (self.x, (self.x * 1.0 / self.period).sin() * self.scale);
        self.x += self.interval;
        Some(point)
    }
}

pub const TAB_NUM: usize = 2;

#[derive(Clone, Copy)]
pub enum TabTeam {
    Home,
    Visitor,
}

pub struct TabsState<'a> {
    pub titles: [&'a str; TAB_NUM],
    pub index: usize,
    pub team: TabTeam,
}

impl<'a> TabsState<'a> {
    pub fn new(titles: [&'a str; TAB_NUM]) -> TabsState {
        TabsState {
            titles,
            index: 0,
            team: TabTeam::Home,
        }
    }
    pub fn next(&mut self) {
        self.index = (self.index + 1) % TAB_NUM;
    }

    pub fn next_team(&mut self) {
        self.team = match self.team {
            TabTeam::Home => TabTeam::Visitor,
            TabTeam::Visitor => TabTeam::Home,
        }
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}
