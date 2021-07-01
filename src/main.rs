mod boxscore;
mod schedule;
mod today_scoreboard;
mod utils;
use clap::{App, AppSettings, Arg, SubCommand};
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
                .setting(AppSettings::ArgRequiredElseHelp)
                .arg(
                    Arg::with_name("game")
                        .conflicts_with("boxscore")
                        .required_unless("boxscore")
                        .help("Print the most up to date information about a game"),
                )
                .arg(
                    Arg::with_name("boxscore")
                        .conflicts_with("game")
                        .required_unless("game")
                        .help("Print the most up to date boxscore of a game"),
                ),
        )
        .subcommand(
            SubCommand::with_name("watch")
                .about("watch the most up to date game info as it updates")
                .setting(AppSettings::ArgRequiredElseHelp)
                .arg(
                    Arg::with_name("game")
                        .conflicts_with("boxscore")
                        .required_unless("boxscore")
                        .help("Watch the game score"),
                )
                .arg(
                    Arg::with_name("boxscore")
                        .conflicts_with("game")
                        .required_unless("game")
                        .help("Watch the boxscore"),
                ),
        )
        .arg(
            Arg::with_name("date")
                .short("d")
                .takes_value(true)
                .required(false)
                .global(true)
                .help("Date of the game in yyyy-mm-dd format, defaults to today"),
        )
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .get_matches();

    let client = reqwest::blocking::Client::new();
    let sc = schedule::Games::new(&client).unwrap();
    let date = matches
        .value_of("date")
        .unwrap_or(&utils::today())
        .to_string();

    match matches.subcommand_name() {
        Some("get") => {
            if matches.is_present("boxscore") {
                let games = sc.get_date_game_id(date.clone());
                games.iter().for_each(|x| {
                    let bs = boxscore::BoxScore::new(&client, date.clone(), &x)
                        .expect("Could not parse json!");
                    bs.print_game();
                });
            }
        }
        Some("watch") => {
            println!("watch")
        }
        _ => (),
    }
}
