mod schedule;
mod utils;
use schedule::Games;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    send_request().await
}

async fn send_request() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let schedules = client
        .get("http://data.nba.com/prod/v1/2020/schedule.json")
        .send()
        .await?
        .text()
        .await?;

    let json = serde_json::from_str::<Games>(&schedules)?;
    json.print_today();

    Ok(())
}
