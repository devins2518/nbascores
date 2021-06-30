use chrono::prelude::*;

pub fn today() -> String {
    let local: DateTime<Local> = Local::now();

    local.format("%Y%m%d").to_string()
}
