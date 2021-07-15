use serde::de::Deserialize;
use serde_derive::Deserialize;
use std::fmt;

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
        let json = Box::leak::<'lf>(Box::new(
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
        //             .join("src/pbp.json"),
        //     )
        //     .unwrap(),
        // ));

        let pbp = serde_json::from_str::<PlayByPlay<'lf>>(json)?;

        Ok(pbp)
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
            sports_content: Content<'lf>,
        }
        #[derive(Deserialize)]
        #[serde(bound(deserialize = "'de: 'lf"))]
        struct Content<'lf> {
            game: Game<'lf>,
        }
        #[derive(Deserialize)]
        #[serde(bound(deserialize = "'de: 'lf"))]
        struct Game<'lf> {
            play: Option<Vec<Play<'lf>>>,
        }

        let helper = Root::deserialize(deserializer)?;
        let plays = if let Some(x) = helper.sports_content.game.play {
            x
        } else {
            Vec::new()
        };

        Ok(Self { plays })
    }
}

#[derive(Debug)]
pub struct Play<'lf> {
    pub clock: &'lf str,
    pub description: &'lf str,
    pub h_score: u8,
    pub v_score: u8,
    pub period: Period,
}

impl<'lf, 'de> Deserialize<'de> for Play<'lf>
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
            clock: &'lf str,
            description: &'lf str,
            home_score: &'lf str,
            visitor_score: &'lf str,
            period: &'lf str,
        }

        let helper = Root::deserialize(deserializer)?;

        let clock = if helper.clock.is_empty() {
            "12:00"
        } else {
            helper.clock
        };

        Ok(Self {
            clock,
            description: helper.description,
            h_score: helper.home_score.parse().unwrap_or(0),
            v_score: helper.visitor_score.parse().unwrap_or(0),
            period: match helper.period {
                "1" => Period::Q1,
                "2" => Period::Q2,
                "3" => Period::Q3,
                "4" => Period::Q4,
                _ => Period::OT,
            },
        })
    }
}

#[derive(Debug)]
pub enum Period {
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
