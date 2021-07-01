mod boxscore;
mod schedule;
mod today_scoreboard;
mod utils;
use utils::PrettyPrintGame;

fn main() {
    send_request()
}

fn send_request() {
    let client = reqwest::blocking::Client::new();
    let sc = schedule::Games::new(&client).unwrap();
    let date = String::from("20210412");
    let games = sc.get_date_game_id(date.clone());
    games.iter().for_each(|x| {
        let bs = boxscore::BoxScore::new(&client, date.clone(), &x).expect("Could not parse json!");
        bs.print_game();
    });
}
