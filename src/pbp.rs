use std::fmt;

use crate::utils::*;
use serde::de::Deserialize;
use serde_derive::Deserialize;

#[derive(Debug)]
pub struct PlayByPlay<'lf> {
    pub plays: Vec<Play<'lf>>,
}

impl<'lf> PlayByPlay<'lf> {
    pub fn new(
        client: &reqwest::blocking::Client,
        game_date: &str,
        game_id: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let boxscore = Box::leak::<'lf>(Box::new(
            client
                .get(format!(
                    "http://data.nba.com/data/10s/json/cms/noseason/game/{}/{}/pbp_all.json",
                    game_date, game_id
                ))
                .send()?
                .text()?,
        ));
        // let boxscore = Box::leak::<'lf>(Box::new(
        //     std::fs::read_to_string(
        //         std::path::PathBuf::from(format!("{}", std::env!("CARGO_MANIFEST_DIR")))
        //             .join("src/playbyplay.json"),
        //     )
        //     .unwrap(),
        // ));

        let boxscore = serde_json::from_str::<PlayByPlay<'lf>>(boxscore)?;

        Ok(boxscore)
    }
}

impl<'lf, 'de> Deserialize<'de> for PlayByPlay<'lf>
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

struct Play<'lf> {
    clock: &'lf str,
    description: &'lf str,
    h_score: u8,
    v_score: u8,
    period: Period,
}

enum Period {
    Q1,
    Q2,
    Q3,
    Q4,
    OT,
}

impl fmt::Display for Period {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Period::Q1 => f.write_str("Q1"),
            Period::Q2 => f.write_str("Q2"),
            Period::Q3 => f.write_str("Q3"),
            Period::Q4 => f.write_str("Q4"),
            Period::OT => f.write_str("OT"),
        }
    }
}
