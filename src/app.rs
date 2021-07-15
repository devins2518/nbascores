use crate::boxscore::BoxScore;
use crate::pbp::{Play, PlayByPlay};
use crate::utils::{RandomSignal, SinSignal, StatefulList, TabTeam, TabsState};

pub struct Signal<S: Iterator> {
    source: S,
    pub points: Vec<S::Item>,
    tick_rate: usize,
}

impl<S> Signal<S>
where
    S: Iterator,
{
    fn on_tick(&mut self) {
        for _ in 0..self.tick_rate {
            self.points.remove(0);
        }
        self.points
            .extend(self.source.by_ref().take(self.tick_rate));
    }
}

pub struct Signals {
    pub sin1: Signal<SinSignal>,
    pub sin2: Signal<SinSignal>,
    pub window: [f64; 2],
}

impl Signals {
    fn on_tick(&mut self) {
        self.sin1.on_tick();
        self.sin2.on_tick();
        self.window[0] += 1.0;
        self.window[1] += 1.0;
    }
}

pub struct App<'a> {
    pub title: &'a str,
    pub tabs: TabsState<'a>,
    pub _show_chart: bool,
    pub _progress: f64,
    pub _sparkline: Signal<RandomSignal>,
    pub _signals: Signals,
    pub boxscore: BoxScore<'a>,
    pub enhanced_graphics: bool,
    pub plays: StatefulList<Play<'a>>,
}

impl<'a> App<'a> {
    pub fn new(
        title: &'a str,
        enhanced_graphics: bool,
        boxscore: BoxScore<'a>,
        playbyplay: PlayByPlay<'a>,
    ) -> App<'a> {
        let mut rand_signal = RandomSignal::new();
        let sparkline_points = rand_signal.by_ref().take(300).collect();
        let mut sin_signal = SinSignal::new(0.2, 3.0, 18.0);
        let sin1_points = sin_signal.by_ref().take(100).collect();
        let mut sin_signal2 = SinSignal::new(0.1, 2.0, 10.0);
        let sin2_points = sin_signal2.by_ref().take(200).collect();
        App {
            title,
            tabs: TabsState::new(["Game", "Boxscore"]),
            _show_chart: true,
            _progress: 0.0,
            _sparkline: Signal {
                source: rand_signal,
                points: sparkline_points,
                tick_rate: 1,
            },
            _signals: Signals {
                sin1: Signal {
                    source: sin_signal,
                    points: sin1_points,
                    tick_rate: 5,
                },
                sin2: Signal {
                    source: sin_signal2,
                    points: sin2_points,
                    tick_rate: 10,
                },
                window: [0.0, 20.0],
            },
            boxscore,
            enhanced_graphics,
            plays: StatefulList::with_items(playbyplay.plays),
        }
    }

    pub fn on_up(&mut self) {
        self.plays.previous();
    }

    pub fn on_down(&mut self) {
        self.plays.next();
    }

    pub fn on_right(&mut self) {
        self.tabs.next();
    }

    pub fn next_team(&mut self) {
        if self.tabs.index == 1 {
            self.tabs.next_team();
        }
    }

    pub fn on_left(&mut self) {
        self.tabs.previous();
    }

    pub fn on_key(&mut self, c: char) {
        match c {
            't' => self._show_chart = !self._show_chart,
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {
        // TODO update boxscore
        // Update progress
        self._progress += 0.001;
        if self._progress > 1.0 {
            self._progress = 0.0;
        }

        self._sparkline.on_tick();
        self._signals.on_tick();
    }

    pub fn get_current_team(&self) -> &str {
        match self.tabs.team {
            TabTeam::Home => self.boxscore.h_team.team_id,
            TabTeam::Visitor => self.boxscore.v_team.team_id,
        }
    }
}
