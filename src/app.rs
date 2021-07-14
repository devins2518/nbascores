use crate::boxscore::BoxScore;
use crate::utils::{RandomSignal, SinSignal, StatefulList, TabTeam, TabsState};

const TASKS: [&str; 24] = [
    "Item1", "Item2", "Item3", "Item4", "Item5", "Item6", "Item7", "Item8", "Item9", "Item10",
    "Item11", "Item12", "Item13", "Item14", "Item15", "Item16", "Item17", "Item18", "Item19",
    "Item20", "Item21", "Item22", "Item23", "Item24",
];

const LOGS: [(&str, &str); 26] = [
    ("Event1", "INFO"),
    ("Event2", "INFO"),
    ("Event3", "CRITICAL"),
    ("Event4", "ERROR"),
    ("Event5", "INFO"),
    ("Event6", "INFO"),
    ("Event7", "WARNING"),
    ("Event8", "INFO"),
    ("Event9", "INFO"),
    ("Event10", "INFO"),
    ("Event11", "CRITICAL"),
    ("Event12", "INFO"),
    ("Event13", "INFO"),
    ("Event14", "INFO"),
    ("Event15", "INFO"),
    ("Event16", "INFO"),
    ("Event17", "ERROR"),
    ("Event18", "ERROR"),
    ("Event19", "INFO"),
    ("Event20", "INFO"),
    ("Event21", "WARNING"),
    ("Event22", "INFO"),
    ("Event23", "INFO"),
    ("Event24", "WARNING"),
    ("Event25", "INFO"),
    ("Event26", "INFO"),
];

const EVENTS: [(&str, u64); 24] = [
    ("B1", 9),
    ("B2", 12),
    ("B3", 5),
    ("B4", 8),
    ("B5", 2),
    ("B6", 4),
    ("B7", 5),
    ("B8", 9),
    ("B9", 14),
    ("B10", 15),
    ("B11", 1),
    ("B12", 0),
    ("B13", 4),
    ("B14", 6),
    ("B15", 4),
    ("B16", 6),
    ("B17", 4),
    ("B18", 7),
    ("B19", 13),
    ("B20", 8),
    ("B21", 11),
    ("B22", 9),
    ("B23", 3),
    ("B24", 5),
];

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
    pub _tasks: StatefulList<&'a str>,
    pub _logs: StatefulList<(&'a str, &'a str)>,
    pub _signals: Signals,
    pub _barchart: Vec<(&'a str, u64)>,
    pub boxscore: BoxScore<'a>,
    pub enhanced_graphics: bool,
    // pub selected: ,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, enhanced_graphics: bool, boxscore: BoxScore<'a>) -> App<'a> {
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
            _tasks: StatefulList::with_items(TASKS.to_vec()),
            _logs: StatefulList::with_items(LOGS.to_vec()),
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
            _barchart: EVENTS.to_vec(),
            boxscore,
            enhanced_graphics,
        }
    }

    pub fn on_up(&mut self) {
        self._tasks.previous();
    }

    pub fn on_down(&mut self) {
        self._tasks.next();
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
        // Update progress
        self._progress += 0.001;
        if self._progress > 1.0 {
            self._progress = 0.0;
        }

        self._sparkline.on_tick();
        self._signals.on_tick();

        let log = self._logs.items.pop().unwrap();
        self._logs.items.insert(0, log);

        let event = self._barchart.pop().unwrap();
        self._barchart.insert(0, event);
    }

    pub fn get_current_team(&self) -> &str {
        match self.tabs.team {
            TabTeam::Home => self.boxscore.h_team.team_id.unwrap(),
            TabTeam::Visitor => self.boxscore.v_team.team_id.unwrap(),
        }
    }
}
