mod app;
mod boxscore;
mod pbp;
mod schedule;
mod ui;
mod utils;
use clap::{App, AppSettings, Arg};
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event as CEvent, KeyCode, KeyModifiers,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    error::Error,
    io::stdout,
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};
use tui::{backend::CrosstermBackend, Terminal};

static VERSION: &str = "0.1";

enum Event<I, J> {
    Input(I),
    MouseEvent(J),
    Tick,
}

fn main() -> Result<(), Box<dyn Error>> {
    let matches = App::new("NBAScores")
        .version(VERSION)
        .author("Devin S. <drsingh2518@icloud.com>")
        .about("Get NBA scores")
        .args(&[
            Arg::with_name("enhanced_graphics")
                .long("enchanced_graphics")
                .short("e")
                .takes_value(true)
                .default_value("true")
                .possible_values(&["true", "false"])
                .help("Use nerd font glyphs."),
            Arg::with_name("tick_rate")
                .long("tick_rate")
                .short("t")
                .takes_value(true)
                .default_value("250")
                .help("Tick rate of the ui."),
            Arg::with_name("date")
                .short("d")
                .long("date")
                .takes_value(true)
                .help("Choose a date in yyyymmdd format. Defaults to today"),
        ])
        .setting(AppSettings::ColoredHelp)
        .get_matches();

    let enhanced_graphics = matches
        .value_of("enhanced_graphics")
        .unwrap()
        .parse()
        .unwrap();

    let client = reqwest::blocking::Client::new();
    let sc = schedule::Schedule::new(&client)
        .expect("Error occured fetching `http://data.nba.com/prod/v1/2020/schedule.json`.");
    let date = matches
        .value_of("date")
        .unwrap_or(&utils::today())
        .to_string();
    let games = sc.get_date_game_id(&date);

    enable_raw_mode()?;

    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;

    // Setup input handling
    let (tx, rx) = mpsc::channel();

    let tick_rate = Duration::from_millis(matches.value_of("tick_rate").unwrap().parse().unwrap());
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            // poll for tick rate duration, if no events, sent tick event.
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));
            if event::poll(timeout).unwrap() {
                match event::read() {
                    Ok(CEvent::Key(key)) => tx.send(Event::Input(key)).unwrap(),
                    Ok(CEvent::Mouse(mouse_event)) => {
                        tx.send(Event::MouseEvent(mouse_event)).unwrap()
                    }
                    _ => (),
                }
            }

            if last_tick.elapsed() >= tick_rate {
                tx.send(Event::Tick).unwrap();
                last_tick = Instant::now();
            }
        }
    });

    terminal.clear()?;

    if games.is_empty() {
        loop {
            terminal.draw(|f| ui::draw_empty_games(f))?;
            match rx.recv()? {
                Event::Input(event) => match event.code {
                    KeyCode::Char('q') => {
                        disable_raw_mode()?;
                        execute!(
                            terminal.backend_mut(),
                            LeaveAlternateScreen,
                            DisableMouseCapture
                        )?;
                        terminal.show_cursor()?;
                        break;
                    }
                    _ => (),
                },
                _ => (),
            }
        }
    } else {
        let boxscore = boxscore::BoxScore::new(&client, &date, games[0]).unwrap_or_else(|_| {
            panic!(
                "Error occured fetching `http://data.nba.com/prod/v1/{}/{}_boxscore.json`",
                date, games[0],
            )
        });
        let pbp = pbp::PlayByPlay::new(&client, &date, games[0]).unwrap_or_else(|_| {
            panic!("Error occured fetching `http://data.nba.com/data/10s/json/cms/noseason/game/{}/{}/pbp_all.json`",
            date, games[0])});

        let mut app = app::App::new("NBAScores", enhanced_graphics, boxscore, pbp, client);

        loop {
            terminal.draw(|f| ui::draw(f, &mut app))?;
            match rx.recv()? {
                Event::Input(event) => match event.modifiers.intersects(KeyModifiers::SHIFT) {
                    true => match event.code {
                        KeyCode::Left
                        | KeyCode::Char('L')
                        | KeyCode::Right
                        | KeyCode::Char('H') => app.next_team(),
                        _ => {}
                    },
                    false => match event.code {
                        KeyCode::Char('q') => {
                            disable_raw_mode()?;
                            execute!(
                                terminal.backend_mut(),
                                LeaveAlternateScreen,
                                DisableMouseCapture
                            )?;
                            terminal.show_cursor()?;
                            break;
                        }
                        KeyCode::Left | KeyCode::Char('l') => app.on_left(),
                        KeyCode::Up | KeyCode::Char('k') => app.on_up(),
                        KeyCode::Right | KeyCode::Char('h') => app.on_right(),
                        KeyCode::Down | KeyCode::Char('j') => app.on_down(),
                        KeyCode::Char(c) => app.on_key(c),
                        _ => {}
                    },
                },
                // TODO
                Event::MouseEvent(event) => match event.kind {
                    _ => {}
                },
                Event::Tick => app.on_tick(),
            }
        }
    }

    Ok(())
}
