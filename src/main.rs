mod boxscore;
mod schedule;
mod today_scoreboard;
mod utils;

fn main() {
    send_request()
}

fn send_request() {
    let client = reqwest::blocking::Client::new();
    let json = schedule::Games::new(&client).expect("Could not parse json!");
    json.print_today(&client);
}
