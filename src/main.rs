mod boxscore;
mod schedule;
mod today_scoreboard;
mod utils;
use clap::{App, Arg, SubCommand};
use utils::PrettyPrintGame;

static VERSION: &str = "0.1";

fn main() {
    let matches = App::new("NBAScores")
        .version(VERSION)
        .author("Devin S. <drsingh2518@icloud.com>")
        .about("Get NBA scores")
        .subcommand(
            SubCommand::with_name("get")
                .about("print the most up to date data")
                .arg(
                    Arg::with_name("game")
                        .conflicts_with("boxscore")
                        .help("Print the most up to date information about a game"),
                )
                .arg(
                    Arg::with_name("boxscore")
                        .conflicts_with("boxscore")
                        .help("Print the most up to date boxscore of a game"),
                ),
        )
        .subcommand(
            SubCommand::with_name("watch")
                .about("watch the most up to date game info as it updates")
                .arg(
                    Arg::with_name("game")
                        .conflicts_with("boxscore")
                        .help("Watch the game score"),
                )
                .arg(
                    Arg::with_name("boxscore")
                        .conflicts_with("boxscore")
                        .help("Watch the boxscore"),
                ),
        )
        .arg(
            Arg::with_name("date")
                .short("d")
                .default_value("today")
                .global(true)
                .help("Date of the game in yyyy-mm-dd format"),
        )
        .get_matches();

    let client = reqwest::blocking::Client::new();
    let sc = schedule::Games::new(&client).unwrap();
    let date = matches
        .value_of("date")
        .unwrap_or(&utils::today())
        .to_string();
    let games = sc.get_date_game_id(date.clone());
    games.iter().for_each(|x| {
        let bs = boxscore::BoxScore::new(&client, date.clone(), &x).expect("Could not parse json!");
        bs.print_game();
    });
}
