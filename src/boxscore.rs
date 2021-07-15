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

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        #[serde(bound(deserialize = "'de: 'lf"))]
        pub struct BasicGameData<'lf> {
            clock: &'lf str,
            playoffs: Option<Playoffs<'lf>>,
            period: Period,
            pub v_team: Team<'lf>,
            pub h_team: Team<'lf>,
        }
        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        #[serde(bound(deserialize = "'de: 'lf"))]
        pub struct Stats<'lf> {
            times_tied: &'lf str,
            lead_changes: &'lf str,
            v_team: StatTeam<'lf>,
            h_team: StatTeam<'lf>,
            pub active_players: Vec<Player<'lf>>,
        }
        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        #[serde(bound(deserialize = "'de: 'lf"))]
        struct StatTeam<'lf> {
            longest_run: &'lf str,
        }

        let helper = Root::deserialize(deserializer)?;
        let players = if let Some(x) = helper.stats {
            x.active_players
        } else {
            [
                Player::from_team_id(helper.bgd.h_team.team_id),
                Player::from_team_id(helper.bgd.v_team.team_id),
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

#[derive(Debug)]
struct Playoffs<'lf> {
    round_num: &'lf str,
    conf_name: &'lf str,
    series_id: &'lf str,
    completed: bool,
    num_in_series: &'lf str,
    if_necessary: bool,
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
            completed: helper.is_series_completed,
            num_in_series: helper.game_num_in_series,
            if_necessary: helper.is_if_necessary,
            v_team_seed: helper.v_team.seed_num.parse().unwrap_or(0),
            h_team_seed: helper.h_team.seed_num.parse().unwrap_or(0),
        })
    }
}

#[derive(Debug)]
pub struct Team<'lf> {
    pub team_id: &'lf str,
    pub tri_code: &'lf str,
    win: u8,
    loss: u8,
    score: u8,
    linescore: [u8; 4],
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
            tri_code: &'lf str,
            win: &'lf str,
            loss: &'lf str,
            score: &'lf str,
            linescore: Vec<LineScore<'lf>>,
        }

        #[derive(Deserialize, Default, Clone, Copy)]
        struct LineScore<'lf> {
            score: &'lf str,
        }

        let helper = Root::deserialize(deserializer)?;

        Ok(Self {
            team_id: helper.team_id,
            tri_code: helper.tri_code,
            win: helper.win.parse().unwrap_or(0),
            loss: helper.loss.parse().unwrap_or(0),
            score: helper.score.parse().unwrap_or(0),
            linescore: [
                helper
                    .linescore
                    .get(0)
                    .unwrap_or(&LineScore { score: "0" })
                    .score
                    .parse()
                    .unwrap(),
                helper
                    .linescore
                    .get(1)
                    .unwrap_or(&LineScore { score: "0" })
                    .score
                    .parse()
                    .unwrap(),
                helper
                    .linescore
                    .get(2)
                    .unwrap_or(&LineScore { score: "0" })
                    .score
                    .parse()
                    .unwrap(),
                helper
                    .linescore
                    .get(3)
                    .unwrap_or(&LineScore { score: "0" })
                    .score
                    .parse()
                    .unwrap(),
            ],
        })
    }
}
