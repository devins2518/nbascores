use serde::Deserialize;

pub struct Schedule {
    games: Vec<Game>,
}

impl Schedule {
    pub fn get_today() -> Self {}
}

impl<'de> Deserialize<'de> for Schedule {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Root {
            league: Inner,
        }
        #[derive(Deserialize)]
        struct Inner {
            standard: Vec<Game>,
        }

        let helper = Root::deserialize(deserializer)?;
        Ok(Self {
            games: helper.league.standard,
        })
    }
}

#[derive(Deserialize)]
struct Game {}

#[derive(Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
struct Team {
    city: String,
    #[serde(rename(deserialize = "fullName"))]
    name: String,
    #[serde(rename(deserialize = "confName"))]
    conference: Conference,
    tricode: String,
    team_short_name: String,
    div_name: Division,
    nickname: String,
    #[serde(deserialize_with = "str_to_u64")]
    team_id: u64,
    alt_city_name: String,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
enum Division {
    Atlantic,
    Central,
    #[serde(rename(deserialize = "Northwest"))]
    NorthWest,
    Pacific,
    #[serde(rename(deserialize = "Southeast"))]
    SouthEast,
    #[serde(rename(deserialize = "Southwest"))]
    SouthWest,
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
enum Conference {
    East,
    West,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_team_parse() {
        let team = r#"{
            "city": "Atlanta",
            "fullName": "Atlanta Hawks",
            "isNBAFranchise": true,
            "confName": "East",
            "tricode": "ATL",
            "teamShortName": "Atlanta",
            "divName": "Southeast",
            "isAllStar": false,
            "nickname": "Hawks",
            "urlName": "hawks",
            "teamId": "1610612737",
            "altCityName": "Atlanta"
        }"#;

        assert_eq!(
            serde_json::from_str::<Team>(team).unwrap(),
            Team {
                city: String::from("Atlanta"),
                name: String::from("Atlanta Hawks"),
                conference: Conference::East,
                tricode: String::from("ATL"),
                team_short_name: String::from("Atlanta"),
                div_name: Division::SouthEast,
                nickname: String::from("Hawks"),
                team_id: 1610612737,
                alt_city_name: String::from("Atlanta")
            }
        );
    }
}

fn str_to_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    (<&str as serde::Deserialize>::deserialize(deserializer)?)
        .parse::<u64>()
        .map_err(|_| serde::de::Error::custom("Failed to parse to u64"))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_team_parse() {
        let team = r#"{
            "city": "Atlanta",
            "fullName": "Atlanta Hawks",
            "isNBAFranchise": true,
            "confName": "East",
            "tricode": "ATL",
            "teamShortName": "Atlanta",
            "divName": "Southeast",
            "isAllStar": false,
            "nickname": "Hawks",
            "urlName": "hawks",
            "teamId": "1610612737",
            "altCityName": "Atlanta"
        }"#;

        assert_eq!(
            serde_json::from_str::<Team>(team).unwrap(),
            Team {
                city: String::from("Atlanta"),
                name: String::from("Atlanta Hawks"),
                conference: Conference::East,
                tricode: String::from("ATL"),
                team_short_name: String::from("Atlanta"),
                div_name: Division::SouthEast,
                nickname: String::from("Hawks"),
                team_id: 1610612737,
                alt_city_name: String::from("Atlanta")
            }
        );
    }
}
