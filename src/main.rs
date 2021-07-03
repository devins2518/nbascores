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
        .args(&[
            Arg::with_name("action")
                .required(true)
                .takes_value(true)
                .possible_values(&["get", "watch"])
                .help("Choose whether you want to get the info and exit, or watch updated info."),
            Arg::with_name("type")
                .required(true)
                .takes_value(true)
                .possible_values(&["boxscore", "game"])
                .help("Choose whether you retreive overall game info or just the boxscore"),
            Arg::with_name("date")
                .short("d")
                .long("date")
                .takes_value(true)
                .help("Choose a date in yyyymmdd format. Defaults to today"),
        ])
        .setting(AppSettings::ArgRequiredElseHelp)
        .setting(AppSettings::ColoredHelp)
        .get_matches();

    let client = reqwest::blocking::Client::new();
    let sc = schedule::Games::new(&client).unwrap();
    let date = matches
        .value_of("date")
        .unwrap_or(&utils::today())
        .to_string();

    match matches.value_of("action") {
        Some("get") => {
            let games = sc.get_date_game_id(&*date);
            games.iter().for_each(|&x| {
                let bs =
                    boxscore::BoxScore::new(&client, &*date, x).expect("Could not parse json!");
                bs.print_game();
            });
        }
        Some("watch") => {
            println!("watch")
        }
        _ => (),
    }
}
